use boolector::{
    option::{BtorOption, ModelGen},
    Btor, SolverResult, BV,
};
use std::rc::Rc;

use super::{BoolectorExpr, BoolectorSolverContext};
use crate::smt::{Solutions, SolverError};

#[derive(Debug, Clone)]
pub struct BoolectorIncrementalSolver {
    ctx: Rc<Btor>,
}

impl BoolectorIncrementalSolver {
    pub fn new(ctx: &BoolectorSolverContext) -> Self {
        Self {
            ctx: ctx.ctx.clone(),
        }
    }

    fn check_sat_result(&self, sat_result: SolverResult) -> Result<bool, SolverError> {
        match sat_result {
            SolverResult::Sat => Ok(true),
            SolverResult::Unsat => Ok(false),
            SolverResult::Unknown => Err(SolverError::Unknown),
        }
    }

    pub fn get_value(&self, expr: &BoolectorExpr) -> Result<BoolectorExpr, SolverError> {
        let expr = expr.clone().simplify();
        if let Some(_) = expr.get_constant() {
            return Ok(expr.clone());
        }

        self.ctx.set_opt(BtorOption::ModelGen(ModelGen::All));

        let result = || {
            if self.is_sat()? {
                self.is_sat()?;
                let solution = expr.0.get_a_solution().disambiguate();
                let solution = solution.as_01x_str();

                let solution = BoolectorExpr(BV::from_binary_str(self.ctx.clone(), solution));
                Ok(solution)
            } else {
                Err(SolverError::Unsat)
            }
        };
        let result = result();

        self.ctx.set_opt(BtorOption::ModelGen(ModelGen::Disabled));

        result
    }

    pub fn push(&self) {
        self.ctx.push(1);
    }

    pub fn pop(&self) {
        self.ctx.pop(1);
    }

    /// Solve for the current solver state, and returns if the result is satisfiable.
    ///
    /// All asserts and assumes are implicitly combined with a boolean and. Returns true or false,
    /// and [SolverError::Unknown] if the result cannot be determined.
    pub fn is_sat(&self) -> Result<bool, SolverError> {
        let sat_result = self.ctx.sat();
        self.check_sat_result(sat_result)
    }

    /// Solve for the solver state with the assumption of the passed constraint.
    pub fn is_sat_with_constraint(&self, constraint: &BoolectorExpr) -> Result<bool, SolverError> {
        // Assume the constraint, will be forgotten after the next call to `is_sat`.
        constraint.0.assume();
        self.is_sat()
    }

    /// Solve for the solver state with the assumption of the passed constraints.
    pub fn is_sat_with_constraints(
        &self,
        constraints: &[BoolectorExpr],
    ) -> Result<bool, SolverError> {
        for constraint in constraints {
            constraint.0.assume();
        }
        self.is_sat()
    }

    /// Add the constraint to the solver.
    ///
    /// The passed constraint will be implicitly combined with the current state in a boolean `and`.
    /// Asserted constraints cannot be removed.
    pub fn assert(&self, constraint: &BoolectorExpr) {
        constraint.0.assert();
    }

    /// Find solutions to `expr`.
    ///
    /// Returns concrete solutions up to `upper_bound`, the returned [`Solutions`] has variants
    /// for if the number of solution exceeds the upper bound.
    pub fn get_values(
        &self,
        expr: &BoolectorExpr,
        upper_bound: usize,
    ) -> Result<Solutions<BoolectorExpr>, SolverError> {
        let expr = expr.clone().simplify();
        if let Some(_) = expr.get_constant() {
            return Ok(Solutions::Exactly(vec![expr]));
        }

        // Setup before checking for solutions.
        self.push();
        self.ctx.set_opt(BtorOption::ModelGen(ModelGen::All));

        let result = self.get_solutions(expr, upper_bound);

        // Restore solver to initial state.
        self.ctx.set_opt(BtorOption::ModelGen(ModelGen::Disabled));
        self.pop();

        result
    }

    /// Returns `true` if `lhs` and `rhs` must be equal under the current constraints.
    pub fn must_be_equal(
        &self,
        lhs: &BoolectorExpr,
        rhs: &BoolectorExpr,
    ) -> Result<bool, SolverError> {
        // Add the constraint lhs != rhs and invert the results. The only way
        // for `lhs != rhs` to be `false` is that if they are equal.
        let constraint = lhs._ne(rhs);
        let result = self.is_sat_with_constraint(&constraint)?;
        Ok(!result)
    }

    /// Check if `lhs` and `rhs` can be equal under the current constraints.
    pub fn can_equal(&self, lhs: &BoolectorExpr, rhs: &BoolectorExpr) -> Result<bool, SolverError> {
        self.is_sat_with_constraint(&lhs._eq(rhs))
    }

    /// Find solutions to `expr`.
    ///
    /// Returns concrete solutions up to a maximum of `upper_bound`. If more solutions are available
    /// the error [`SolverError::TooManySolutions`] is returned.
    pub fn get_solutions2(
        &self,
        expr: &BoolectorExpr,
        upper_bound: usize,
    ) -> Result<Vec<BoolectorExpr>, SolverError> {
        let result = self.get_values(expr, upper_bound)?;
        match result {
            Solutions::Exactly(solutions) => Ok(solutions),
            Solutions::AtLeast(_) => Err(SolverError::TooManySolutions),
        }
    }

    // TODO: Commpare this against the other... Not sure why there are two.
    fn get_solutions(
        &self,
        expr: BoolectorExpr,
        upper_bound: usize,
    ) -> Result<Solutions<BoolectorExpr>, SolverError> {
        let mut solutions = Vec::new();

        self.ctx.set_opt(BtorOption::ModelGen(ModelGen::All));

        let result = || {
            while solutions.len() < upper_bound && self.is_sat()? {
                let solution = expr.0.get_a_solution().disambiguate();
                let solution = solution.as_01x_str();
                let solution = BoolectorExpr(BV::from_binary_str(self.ctx.clone(), solution));

                // Constrain the next value to not be an already found solution.
                self.assert(&expr._ne(&solution));

                solutions.push(solution);
            }

            let exists_more_solutions = self.is_sat()?;
            match exists_more_solutions {
                false => Ok(Solutions::Exactly(solutions)),
                true => Ok(Solutions::AtLeast(solutions)),
            }
        };
        let result = result();

        self.ctx.set_opt(BtorOption::ModelGen(ModelGen::Disabled));

        result
    }
}

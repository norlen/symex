use boolector::{
    option::{BtorOption, ModelGen},
    Btor, SolverResult, BV,
};
use std::rc::Rc;

use super::{BoolectorExpr, BoolectorSolverContext, Solver, SolverError};
use crate::smt::{Expression, Solutions};

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

impl Solver for BoolectorIncrementalSolver {
    type E = BoolectorExpr;

    fn is_sat(&self) -> Result<bool, SolverError> {
        let sat_result = self.ctx.sat();
        self.check_sat_result(sat_result)
    }

    fn is_sat_with_constraint(&self, constraint: &Self::E) -> Result<bool, SolverError> {
        // Assume the constraint, will be forgotten after the next call to `is_sat`.
        constraint.0.assume();
        self.is_sat()
    }

    fn is_sat_with_constraints(&self, constraints: &[Self::E]) -> Result<bool, SolverError> {
        for constraint in constraints {
            constraint.0.assume();
        }
        self.is_sat()
    }

    fn assert(&self, constraint: &Self::E) {
        constraint.0.assert();
    }

    fn get_values(
        &self,
        expr: &Self::E,
        upper_bound: usize,
    ) -> Result<Solutions<Self::E>, SolverError> {
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
}

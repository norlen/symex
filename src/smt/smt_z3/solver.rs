use std::rc::Rc;
use z3::{
    ast::{Ast, Bool, BV},
    SatResult,
};

use crate::smt::{Expression, Solutions};

use super::{Solver, SolverError, Z3Expr, Z3SolverContext};

#[derive(Debug, Clone)]
pub struct Z3SolverIncremental<'ctx> {
    solver: Rc<z3::Solver<'ctx>>,
}

impl<'ctx> Z3SolverIncremental<'ctx> {
    pub fn new(ctx: &Z3SolverContext<'ctx>) -> Self {
        let solver = Rc::new(z3::Solver::new(ctx.ctx));
        Self { solver }
    }

    fn check_sat_result(&self, sat_result: SatResult) -> Result<bool, SolverError> {
        match sat_result {
            SatResult::Sat => Ok(true),
            SatResult::Unsat => Ok(false),
            SatResult::Unknown => Err(SolverError::Unknown),
        }
    }

    pub fn get_value(&self, expr: &Z3Expr<'ctx>) -> Result<Z3Expr<'ctx>, SolverError> {
        let expr = expr.clone().simplify();
        if let Some(_) = expr.get_constant() {
            return Ok(expr);
        }

        let _is_sat = self.is_sat()?;
        let model = self.solver.get_model();
        let model = model.unwrap();
        let e = match &expr {
            Z3Expr::Bool(b) => model.eval(b, true).unwrap().into(),
            Z3Expr::BV(bv) => model.eval(bv, true).unwrap().into(),
        };

        Ok(e)
    }

    pub fn push(&self) {
        self.solver.push();
    }

    pub fn pop(&self) {
        self.solver.pop(1);
    }
}

impl<'ctx> Solver for Z3SolverIncremental<'ctx> {
    type E = Z3Expr<'ctx>;

    fn is_sat(&self) -> Result<bool, SolverError> {
        let sat_result = self.solver.check();
        self.check_sat_result(sat_result)
    }

    fn is_sat_with_constraint(&self, constraint: &Self::E) -> Result<bool, SolverError> {
        // Assume the constraint, will be forgotten after the next call to `is_sat`.
        // constraint.0.assume();
        let b = constraint.clone().to_bool();
        let sat_result = self.solver.check_assumptions(&[b]);
        self.check_sat_result(sat_result)
    }

    fn is_sat_with_constraints(&self, constraints: &[Self::E]) -> Result<bool, SolverError> {
        let bools = constraints
            .iter()
            .map(|c| c.as_bool().clone())
            .collect::<Vec<Bool<'_>>>();

        let sat_result = self.solver.check_assumptions(&bools);
        self.check_sat_result(sat_result)
    }

    fn assert(&self, constraint: &Self::E) {
        match constraint {
            Z3Expr::Bool(b) => self.solver.assert(b),
            Z3Expr::BV(bv) => {
                let bv_true = BV::from_u64(bv.get_ctx(), 1, bv.get_size());
                let b = bv._eq(&bv_true);
                self.solver.assert(&b);
            }
        };
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

        self.push();

        let mut results = Vec::new();
        while results.len() < upper_bound {
            let is_sat = self.is_sat().unwrap();
            if !is_sat {
                break;
            }
            let model = self.solver.get_model();
            let model = model.unwrap();

            let e: Z3Expr<'ctx> = match &expr {
                Z3Expr::Bool(b) => model.eval(b, true).unwrap().into(),
                Z3Expr::BV(bv) => model.eval(bv, true).unwrap().into(),
            };

            let cond = e._ne(&expr);

            results.push(e);

            self.assert(&cond);
        }

        self.pop();

        let is_sat = self.is_sat().unwrap();
        match is_sat {
            true => Ok(Solutions::Exactly(results)),
            false => Ok(Solutions::AtLeast(results)),
        }
    }
}

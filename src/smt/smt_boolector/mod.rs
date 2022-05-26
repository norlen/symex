use super::{Expression, Solver, SolverContext, SolverError};
use boolector::{
    option::{BtorOption, NumberFormat},
    Btor, BV,
};
use std::rc::Rc;

pub(crate) mod expr;
mod solver;

//pub(super) use builder::Z3Builder;
pub(super) use expr::BoolectorExpr;
pub(super) use solver::BoolectorIncrementalSolver;

#[derive(Debug, Clone)]
pub struct BoolectorSolverContext {
    pub ctx: Rc<Btor>,
}

impl SolverContext<BoolectorExpr> for BoolectorSolverContext {
    type Solver = BoolectorIncrementalSolver;

    fn unconstrained(&self, bits: u32, name: &str) -> BoolectorExpr {
        BoolectorExpr(BV::new(self.ctx.clone(), bits, Some(name)))
    }

    fn one(&self, bits: u32) -> BoolectorExpr {
        BoolectorExpr(boolector::BV::from_u64(self.ctx.clone(), 1, bits))
    }

    fn zero(&self, bits: u32) -> BoolectorExpr {
        BoolectorExpr(boolector::BV::zero(self.ctx.clone(), bits))
    }

    fn from_bool(&self, value: bool) -> BoolectorExpr {
        BoolectorExpr(boolector::BV::from_bool(self.ctx.clone(), value))
    }

    fn from_u64(&self, value: u64, bits: u32) -> BoolectorExpr {
        BoolectorExpr(boolector::BV::from_u64(self.ctx.clone(), value, bits))
    }

    fn from_binary_string(&self, bits: &str) -> BoolectorExpr {
        BoolectorExpr(boolector::BV::from_binary_str(self.ctx.clone(), bits))
    }
}

impl BoolectorSolverContext {
    pub fn new() -> Self {
        let btor = Btor::new();
        let ctx = Rc::new(btor);
        ctx.set_opt(BtorOption::Incremental(true));
        ctx.set_opt(BtorOption::PrettyPrint(true));
        ctx.set_opt(BtorOption::OutputNumberFormat(NumberFormat::Hexadecimal));
        // ctx.set_opt(BtorOption::RewriteLevel(RewriteLevel::Full));
        // ctx.set_opt(BtorOption::SkeletonPreproc(true));
        // ctx.set_opt(BtorOption::Ackermann(true));
        // ctx.set_opt(BtorOption::BetaReduce(true));
        // ctx.set_opt(BtorOption::EliminateSlices(true));
        // ctx.set_opt(BtorOption::VariableSubst(true));
        // ctx.set_opt(BtorOption::MergeLambdas(true));
        // ctx.set_opt(BtorOption::ExtractLambdas(true));
        // ctx.set_opt(BtorOption::Normalize(true));
        // ctx.set_opt(BtorOption::NormalizeAdd(true));

        Self { ctx }
    }
}

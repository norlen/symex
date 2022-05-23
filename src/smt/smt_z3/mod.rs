use z3::{
    ast::{Bool, BV},
    Context,
};

use super::{Expression, Solver, SolverContext, SolverError};

//mod builder;
mod expr;
mod solver;

//pub(super) use builder::Z3Builder;
pub(super) use expr::Z3Expr;
pub(super) use solver::Z3SolverIncremental;

#[derive(Debug, Clone)]
pub struct Z3SolverContext<'ctx> {
    pub ctx: &'ctx Context,
}

impl<'ctx> SolverContext<Z3Expr<'ctx>> for Z3SolverContext<'ctx> {
    //type Builder = Z3Builder<'static>;
    type Solver = Z3SolverIncremental<'ctx>;
    // type E = Z3Expr<'static>;

    fn unconstrained(&self, bits: u32, name: &str) -> Z3Expr<'ctx> {
        BV::new_const(self.ctx, name, bits).into()
    }

    fn one(&self, bits: u32) -> Z3Expr<'ctx> {
        BV::from_u64(self.ctx, 1, bits).into()
    }

    fn zero(&self, bits: u32) -> Z3Expr<'ctx> {
        BV::from_u64(self.ctx, 0, bits).into()
    }

    fn from_bool(&self, value: bool) -> Z3Expr<'ctx> {
        Bool::from_bool(self.ctx, value).into()
    }

    fn from_u64(&self, value: u64, bits: u32) -> Z3Expr<'ctx> {
        BV::from_u64(self.ctx, value, bits).into()
    }

    fn from_binary_string(&self, bits: &str) -> Z3Expr<'ctx> {
        BV::fresh_const(self.ctx, bits, bits.len() as u32).into()
    }
}

impl<'ctx> Z3SolverContext<'ctx> {
    pub fn new() -> Self {
        let cfg = z3::Config::new();
        let ctx = Box::new(Context::new(&cfg));
        let ctx = Box::leak(ctx);

        Self { ctx }
    }
}

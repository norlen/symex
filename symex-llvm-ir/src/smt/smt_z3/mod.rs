use z3::{
    ast::{Ast, Bool, BV},
    Context,
};

use crate::core::smt::{self, SolverContext};

mod expr;
mod solver;

// Re-exports
pub(super) use expr::Z3Expr;
pub(super) use solver::Z3SolverIncremental;

#[derive(Debug, Clone)]
pub struct Z3SolverContext<'ctx> {
    pub ctx: &'ctx Context,
}

impl<'ctx> SolverContext<Z3Expr<'ctx>> for Z3SolverContext<'ctx> {
    type Solver = Z3SolverIncremental<'ctx>;

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
        // TODO: Clean up this code.
        let mut res: Option<BV<'ctx>> = None;
        let mut curr = 0;
        while curr < bits.len() {
            let upper_bound = std::cmp::min(bits.len(), curr + 64);
            let v = &bits[curr..upper_bound];
            let v = i64::from_str_radix(v, 2).unwrap();
            let width = upper_bound - curr;
            let v = BV::from_i64(self.ctx, v, width as u32);
            match res {
                Some(c) => res = Some(c.concat(&v)),
                None => res = Some(v),
            }
            curr = upper_bound;
        }
        let res = res.unwrap().simplify();
        Z3Expr::BV(res)
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

#[derive(Debug, Clone)]
pub struct Z3Array<'ctx>(z3::ast::Array<'ctx>);

impl<'ctx> smt::Array for Z3Array<'ctx> {
    type Expression = Z3Expr<'ctx>;

    type Context = Z3SolverContext<'ctx>;

    fn new(ctx: &Self::Context, index_size: usize, element_size: usize, name: &str) -> Self {
        let index_sort = z3::Sort::bitvector(&ctx.ctx, index_size as u32);
        let element_sort = z3::Sort::bitvector(&ctx.ctx, element_size as u32);
        let arr = z3::ast::Array::new_const(&ctx.ctx, name, &index_sort, &element_sort);
        Self(arr)
    }

    fn read(&self, index: &Self::Expression) -> Self::Expression {
        let index = index.coerce_bv();
        if let Some(bv) = self.0.select(&index).as_bv() {
            Z3Expr::BV(bv)
        } else {
            panic!();
        }
    }

    fn write(&mut self, index: &Self::Expression, value: Self::Expression) {
        let index = index.coerce_bv();
        let value = value.coerce_bv();
        self.0 = self.0.store(&index, &value);
    }
}

use z3::Context;
use z3::{ast, ast::Ast};

use super::super::{Array, BinaryOperation, Bitvector, Bool, Float, Solutions, Solver};

pub struct Z3Solver<'ctx> {
    ctx: &'ctx Context,

    solver: z3::Solver<'ctx>,
}

impl<'ctx> Solver for Z3Solver<'ctx> {
    type Bitvector = BV<'ctx>;

    type Array = Z3Array;

    type Bool = Z3Bool<'ctx>;

    type Float = Z3Float<'ctx>;

    fn new() -> Self {
        todo!()
    }

    fn push(&self) {
        self.solver.push()
    }

    fn pop(&self) {
        self.solver.pop(1)
    }

    fn is_sat(&self) -> anyhow::Result<bool> {
        match self.solver.check() {
            z3::SatResult::Unsat => Ok(false),
            z3::SatResult::Sat => Ok(true),
            z3::SatResult::Unknown => Err(anyhow::anyhow!("solver result unknown")),
        }
    }

    fn is_sat_with_constraint(&self, constraint: &Self::Bool) -> anyhow::Result<bool> {
        self.push();

        self.assert(constraint);
        let is_sat = self.is_sat();

        self.pop();

        is_sat
    }

    fn array(&self, index_width: u32, element_width: u32, symbol: Option<&str>) -> Self::Array {
        let domain = z3::Sort::bitvector(&self.ctx, index_width);
        let range = z3::Sort::bitvector(&self.ctx, element_width);
        // Self::Array::new_const(&self.ctx, symbol.unwrap(), &domain, &range)
        todo!()
    }

    fn bv(&self, bits: u32) -> Self::Bitvector {
        todo!()
    }

    fn bv_from_bool(&self, value: bool) -> Self::Bitvector {
        todo!()
    }

    fn bv_from_u64(&self, value: u64, bits: u32) -> Self::Bitvector {
        BV(z3::ast::BV::from_u64(&self.ctx, value, bits))
    }

    fn bv_zero(&self, bits: u32) -> Self::Bitvector {
        todo!()
    }

    fn from_binary_string(&self, bits: &str) -> Self::Bitvector {
        todo!()
    }

    fn assert(&self, constraint: &Self::Bool) {
        self.solver.assert(&constraint.0);
    }
}

#[derive(Clone)]
pub struct BV<'ctx>(ast::BV<'ctx>);

impl<'ctx> BV<'ctx> {}

impl<'ctx> Bitvector for BV<'ctx> {
    type Solver = Z3Solver<'ctx>;

    fn len(&self) -> u32 {
        self.0.get_size()
    }

    fn zero_ext(&self, width: u32) -> Self {
        match self.len().cmp(&width) {
            std::cmp::Ordering::Less => BV(self.0.zero_ext(width - self.len())),
            std::cmp::Ordering::Equal => self.clone(),
            std::cmp::Ordering::Greater => todo!(),
        }
    }

    fn sign_ext(&self, width: u32) -> Self {
        match self.len().cmp(&width) {
            std::cmp::Ordering::Less => BV(self.0.sign_ext(width - self.len())),
            std::cmp::Ordering::Equal => self.clone(),
            std::cmp::Ordering::Greater => todo!(),
        }
    }

    fn binary_op(&self, other: Self, binop: BinaryOperation) -> Self {
        match binop {
            BinaryOperation::Add => BV(self.0.bvadd(&other.0)),
            BinaryOperation::Sub => BV(self.0.bvsub(&other.0)),
            BinaryOperation::Mul => BV(self.0.bvmul(&other.0)),
            BinaryOperation::UDiv => BV(self.0.bvudiv(&other.0)),
            BinaryOperation::SDiv => BV(self.0.bvsdiv(&other.0)),
            BinaryOperation::URem => BV(self.0.bvurem(&other.0)),
            BinaryOperation::SRem => BV(self.0.bvsrem(&other.0)),
        }
    }

    fn eq(&self, other: &Self) -> <<Self as Bitvector>::Solver as Solver>::Bool {
        Z3Bool(self.0._eq(&other.0))
    }

    fn ne(&self, other: &Self) -> <<Self as Bitvector>::Solver as Solver>::Bool {
        Z3Bool(self.0._eq(&other.0).not())
    }

    fn add(&self, other: &Self) -> Self {
        BV(self.0.bvadd(&other.0))
    }

    fn sub(&self, other: &Self) -> Self {
        BV(self.0.bvsub(&other.0))
    }

    fn mul(&self, other: &Self) -> Self {
        BV(self.0.bvmul(&other.0))
    }

    fn udiv(&self, other: &Self) -> Self {
        BV(self.0.bvudiv(&other.0))
    }

    fn sdiv(&self, other: &Self) -> Self {
        BV(self.0.bvsdiv(&other.0))
    }

    fn urem(&self, other: &Self) -> Self {
        BV(self.0.bvurem(&other.0))
    }

    fn srem(&self, other: &Self) -> Self {
        BV(self.0.bvsrem(&other.0))
    }
}

#[derive(Clone)]
pub struct Z3Array(u64);

impl Array for Z3Array {}

#[derive(Clone)]
pub struct Z3Bool<'ctx>(z3::ast::Bool<'ctx>);

impl<'ctx> Bool for Z3Bool<'ctx> {}

#[derive(Clone)]
pub struct Z3Float<'ctx>(z3::ast::Float<'ctx>);

impl<'ctx> Float for Z3Float<'ctx> {}

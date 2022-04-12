use anyhow::{anyhow, Result};
use boolector::{
    option::{BtorOption, ModelGen, NumberFormat},
    Btor, SolverResult,
};
use std::rc::Rc;

use super::{Array, BinaryOperation, Bitvector, Bool, Float, Solutions, Solver};

#[derive(Clone)]
pub struct BoolectorSolver {
    btor: Rc<Btor>,
}

impl Solver for BoolectorSolver {
    type Bitvector = BV;

    type Array = BoolectorArray;

    type Bool = BV;

    type Float = BV;

    fn new() -> Self {
        let btor = Btor::new();
        btor.set_opt(BtorOption::Incremental(true));
        btor.set_opt(BtorOption::PrettyPrint(true));
        btor.set_opt(BtorOption::OutputNumberFormat(NumberFormat::Hexadecimal));

        Self {
            btor: Rc::new(btor),
        }
    }

    fn push(&self) {
        self.btor.push(1)
    }

    fn pop(&self) {
        self.btor.pop(1)
    }

    fn is_sat(&self) -> anyhow::Result<bool> {
        match self.btor.sat() {
            SolverResult::Sat => Ok(true),
            SolverResult::Unsat => Ok(false),
            SolverResult::Unknown => Err(anyhow!("solver result unknown")),
        }
    }

    fn is_sat_with_constraint(&self, constraint: &Self::Bool) -> anyhow::Result<bool> {
        self.btor.push(1);

        self.assert(constraint);

        let is_sat = self.is_sat()?;
        self.btor.pop(1);
        Ok(is_sat)
    }

    fn array(&self, index_width: u32, element_width: u32, symbol: Option<&str>) -> Self::Array {
        BoolectorArray(boolector::Array::new(
            self.btor.clone(),
            index_width,
            element_width,
            symbol,
        ))
    }

    fn bv(&self, bits: u32) -> Self::Bitvector {
        BV(boolector::BV::new(self.btor.clone(), bits, None).into())
    }

    fn bv_from_bool(&self, value: bool) -> Self::Bitvector {
        BV(boolector::BV::from_bool(self.btor.clone(), value).into())
    }

    fn bv_from_u64(&self, value: u64, bits: u32) -> Self::Bitvector {
        BV(boolector::BV::from_u64(self.btor.clone(), value, bits).into())
    }

    fn bv_zero(&self, bits: u32) -> Self::Bitvector {
        BV(boolector::BV::zero(self.btor.clone(), bits).into())
    }

    fn from_binary_string(&self, bits: &str) -> Self::Bitvector {
        BV(boolector::BV::from_binary_str(self.btor.clone(), bits).into())
    }

    fn assert(&self, constraint: &Self::Bool) {
        constraint.assert();
    }
}

#[derive(Clone)]
pub struct BV(boolector::BV<Rc<Btor>>);

impl BV {
    fn assert(&self) {
        todo!()
    }
}

impl Bitvector for BV {
    type Solver = BoolectorSolver;

    fn len(&self) -> u32 {
        self.0.get_width()
    }

    fn zero_ext(&self, width: u32) -> Self {
        let w = self.0.get_width();

        match w.cmp(&width) {
            std::cmp::Ordering::Less => BV(self.0.uext(width - w)),
            std::cmp::Ordering::Equal => self.clone(),
            std::cmp::Ordering::Greater => todo!(),
        }
    }

    fn sign_ext(&self, width: u32) -> Self {
        let w = self.0.get_width();

        match w.cmp(&width) {
            std::cmp::Ordering::Less => BV(self.0.sext(width - w)),
            std::cmp::Ordering::Equal => self.clone(),
            std::cmp::Ordering::Greater => todo!(),
        }
    }

    fn binary_op(&self, other: Self, binop: super::BinaryOperation) -> Self {
        match binop {
            BinaryOperation::Add => self.add(&other),
            BinaryOperation::Sub => self.sub(&other),
            BinaryOperation::Mul => self.mul(&other),
            BinaryOperation::UDiv => self.udiv(&other),
            BinaryOperation::SDiv => self.sdiv(&other),
            BinaryOperation::URem => self.urem(&other),
            BinaryOperation::SRem => self.srem(&other),
        }
    }

    fn eq(&self, other: &Self) -> <<Self as Bitvector>::Solver as Solver>::Bool {
        BV(self.0._eq(&other.0))
    }

    fn ne(&self, other: &Self) -> <<Self as Bitvector>::Solver as Solver>::Bool {
        BV(self.0._ne(&other.0))
    }

    fn add(&self, other: &Self) -> Self {
        BV(self.0.add(&other.0))
    }

    fn sub(&self, other: &Self) -> Self {
        BV(self.0.sub(&other.0))
    }

    fn mul(&self, other: &Self) -> Self {
        BV(self.0.mul(&other.0))
    }

    fn udiv(&self, other: &Self) -> Self {
        BV(self.0.udiv(&other.0))
    }

    fn sdiv(&self, other: &Self) -> Self {
        BV(self.0.sdiv(&other.0))
    }

    fn urem(&self, other: &Self) -> Self {
        BV(self.0.urem(&other.0))
    }

    fn srem(&self, other: &Self) -> Self {
        BV(self.0.srem(&other.0))
    }
}

impl Bool for BV {}

impl Float for BV {}

#[derive(Clone)]
pub struct BoolectorArray(boolector::Array<Rc<Btor>>);

impl Array for BoolectorArray {}

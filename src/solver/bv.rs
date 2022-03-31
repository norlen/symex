use boolector::{BVSolution, Btor};
use std::convert::From;
use std::{ops::Deref, rc::Rc};

use super::{BinaryOperation, UnaryOperation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BV(pub(super) boolector::BV<Rc<Btor>>);

impl BV {
    pub fn zero_ext(&self, width: u32) -> BV {
        let w = self.0.get_width();

        match w.cmp(&width) {
            std::cmp::Ordering::Less => self.0.uext(width - w).into(),
            std::cmp::Ordering::Equal => self.clone(),
            std::cmp::Ordering::Greater => todo!(),
        }
    }

    pub fn sign_ext(&self, width: u32) -> BV {
        let w = self.0.get_width();

        match w.cmp(&width) {
            std::cmp::Ordering::Less => self.0.sext(width - w).into(),
            std::cmp::Ordering::Equal => self.clone(),
            std::cmp::Ordering::Greater => todo!(),
        }
    }

    pub fn binary_op(&self, other: BV, binop: BinaryOperation) -> Self {
        match binop {
            BinaryOperation::Add => self.add(&other),
            BinaryOperation::Sub => self.sub(&other),
            BinaryOperation::Mul => self.mul(&other).into(),
            BinaryOperation::UDiv => self.udiv(&other).into(),
            BinaryOperation::SDiv => self.sdiv(&other).into(),
            BinaryOperation::URem => self.urem(&other).into(),
            BinaryOperation::SRem => self.srem(&other).into(),
        }
    }

    pub fn unary_op(&self, _unary_op: UnaryOperation) -> Self {
        todo!()
    }

    pub fn get_width(&self) -> u32 {
        self.0.get_width()
    }

    pub fn assert(&self) {
        self.0.assert();
    }

    pub fn get_solution(&self) -> BVSolution {
        self.0.get_a_solution()
    }

    pub fn _ne(&self, other: &BV) -> BV {
        self.0._ne(&other.0).into()
    }

    pub fn _eq(&self, other: &BV) -> BV {
        self.0._eq(&other.0).into()
    }

    pub fn add(&self, other: &BV) -> BV {
        self.0.add(&other.0).into()
    }

    pub fn sub(&self, other: &BV) -> BV {
        self.0.sub(&other.0).into()
    }
}

impl Deref for BV {
    type Target = boolector::BV<Rc<Btor>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<boolector::BV<Rc<Btor>>> for BV {
    fn from(bv: boolector::BV<Rc<Btor>>) -> Self {
        Self(bv)
    }
}

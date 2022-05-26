#![allow(clippy::len_without_is_empty)]
use boolector::{Btor, BV};
use std::{cmp::Ordering, rc::Rc};

use super::{BoolectorSolverContext, Expression};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolectorExpr(pub(crate) BV<Rc<Btor>>);

impl Expression for BoolectorExpr {
    type Context = BoolectorSolverContext;

    fn len(&self) -> u32 {
        self.0.get_width()
    }

    fn zero_ext(&self, width: u32) -> Self {
        assert!(self.len() <= width);
        match self.len().cmp(&width) {
            Ordering::Less => BoolectorExpr(self.0.uext(width - self.len())),
            Ordering::Equal => self.clone(),
            Ordering::Greater => todo!(),
        }
    }

    fn sign_ext(&self, width: u32) -> Self {
        assert!(self.len() <= width);
        match self.len().cmp(&width) {
            Ordering::Less => BoolectorExpr(self.0.sext(width - self.len())),
            Ordering::Equal => self.clone(),
            Ordering::Greater => todo!(),
        }
    }

    fn resize_unsigned(&self, width: u32) -> Self {
        match self.len().cmp(&width) {
            Ordering::Equal => self.clone(),
            Ordering::Less => self.zero_ext(width),
            Ordering::Greater => self.slice(0, width - 1),
        }
    }

    fn _eq(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0._eq(&other.0))
    }

    fn _ne(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0._ne(&other.0))
    }

    fn ugt(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.ugt(&other.0))
    }

    fn ugte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.ugte(&other.0))
    }

    fn ult(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.ult(&other.0))
    }

    fn ulte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.ulte(&other.0))
    }

    fn sgt(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.sgt(&other.0))
    }

    fn sgte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.sgte(&other.0))
    }

    fn slt(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.slt(&other.0))
    }

    fn slte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.slte(&other.0))
    }

    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.add(&other.0))
    }

    fn sub(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.sub(&other.0))
    }

    fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.mul(&other.0))
    }

    fn udiv(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.udiv(&other.0))
    }

    fn sdiv(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.sdiv(&other.0))
    }

    fn urem(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.urem(&other.0))
    }

    fn srem(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.srem(&other.0))
    }

    fn not(&self) -> Self {
        Self(self.0.not())
    }

    fn and(&self, other: &Self) -> Self {
        Self(self.0.and(&other.0))
    }

    fn or(&self, other: &Self) -> Self {
        Self(self.0.or(&other.0))
    }

    fn xor(&self, other: &Self) -> Self {
        Self(self.0.xor(&other.0))
    }

    fn sll(&self, other: &Self) -> Self {
        Self(self.0.sll(&other.0))
    }

    fn srl(&self, other: &Self) -> Self {
        Self(self.0.srl(&other.0))
    }

    fn sra(&self, other: &Self) -> Self {
        Self(self.0.sra(&other.0))
    }

    fn ite(&self, then_bv: &Self, else_bv: &Self) -> Self {
        assert_eq!(self.len(), 1);
        Self(self.0.cond_bv(&then_bv.0, &else_bv.0))
    }

    fn concat(&self, other: &Self) -> Self {
        Self(self.0.concat(&other.0))
    }

    fn slice(&self, low: u32, high: u32) -> Self {
        assert!(low <= high);
        assert!(high <= self.len());
        Self(self.0.slice(high, low))
    }

    fn uaddo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.uaddo(&other.0))
    }

    fn saddo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.saddo(&other.0))
    }

    fn usubo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.usubo(&other.0))
    }

    fn ssubo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.ssubo(&other.0))
    }

    fn umulo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.umulo(&other.0))
    }

    fn smulo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.smulo(&other.0))
    }

    fn simplify(self) -> Self {
        self
    }

    fn get_constant(&self) -> Option<u64> {
        if let Some(binary_str) = self.0.as_binary_str() {
            let num = u64::from_str_radix(&binary_str, 2).unwrap();
            Some(num)
        } else {
            None
        }
    }

    fn get_constant_bool(&self) -> Option<bool> {
        assert_eq!(self.len(), 1);
        if let Some(binary_str) = self.0.as_binary_str() {
            Some(binary_str != "0")
        } else {
            None
        }
    }

    fn to_binary_string(&self) -> String {
        // TODO: Check if there's a better way to get the an underlying string.
        if self.len() <= 64 {
            format!("{:b}", self.get_constant().unwrap())
        } else {
            let upper = self.slice(64, self.len() - 1).to_binary_string();
            let lower = self.slice(0, 63).to_binary_string();
            format!("{}{}", upper, lower)
        }
    }

    fn get_ctx(&self) -> Self::Context {
        let ctx = self.0.get_btor();
        BoolectorSolverContext { ctx }
    }
}

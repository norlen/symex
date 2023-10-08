#![allow(clippy::len_without_is_empty)]
use boolector::{Btor, BV};
use std::{cmp::Ordering, rc::Rc};

use super::BoolectorSolverContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolectorExpr(pub(crate) BV<Rc<Btor>>);

impl BoolectorExpr {
    /// Returns the bit width of the [Expression].
    pub fn len(&self) -> u32 {
        self.0.get_width()
    }

    /// Zero-extend the current [Expression] to the passed bit width and return the resulting
    /// [Expression].
    pub fn zero_ext(&self, width: u32) -> Self {
        assert!(self.len() <= width);
        match self.len().cmp(&width) {
            Ordering::Less => BoolectorExpr(self.0.uext(width - self.len())),
            Ordering::Equal => self.clone(),
            Ordering::Greater => todo!(),
        }
    }

    /// Sign-extend the current [Expression] to the passed bit width and return the resulting
    /// [Expression].
    pub fn sign_ext(&self, width: u32) -> Self {
        assert!(self.len() <= width);
        match self.len().cmp(&width) {
            Ordering::Less => BoolectorExpr(self.0.sext(width - self.len())),
            Ordering::Equal => self.clone(),
            Ordering::Greater => todo!(),
        }
    }

    pub fn resize_unsigned(&self, width: u32) -> Self {
        match self.len().cmp(&width) {
            Ordering::Equal => self.clone(),
            Ordering::Less => self.zero_ext(width),
            Ordering::Greater => self.slice(0, width - 1),
        }
    }

    /// [Expression] equality check. Both [Expression]s must have the same bit width, the result is
    /// returned as an [Expression] of width `1`.
    pub fn _eq(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0._eq(&other.0))
    }

    /// [Expression] inequality check. Both [Expression]s must have the same bit width, the result is
    /// returned as an [Expression] of width `1`.
    pub fn _ne(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0._ne(&other.0))
    }

    /// [Expression] unsigned greater than. Both [Expression]s must have the same bit width, the
    /// result is returned as an [Expression] of width `1`.
    pub fn ugt(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.ugt(&other.0))
    }

    /// [Expression] unsigned greater than or equal. Both [Expression]s must have the same bit
    /// width, the result is returned as an [Expression] of width `1`.
    pub fn ugte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.ugte(&other.0))
    }

    /// [Expression] unsigned less than. Both [Expression]s must have the same bit width, the result
    /// is returned as an [Expression] of width `1`.
    pub fn ult(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.ult(&other.0))
    }

    /// [Expression] unsigned less than or equal. Both [Expression]s must have the same bit width,
    /// the result is returned as an [Expression] of width `1`.
    pub fn ulte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.ulte(&other.0))
    }

    /// [Expression] signed greater than. Both [Expression]s must have the same bit width, the
    /// result is returned as an [Expression] of width `1`.
    pub fn sgt(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.sgt(&other.0))
    }

    /// [Expression] signed greater or equal than. Both [Expression]s must have the same bit width,
    /// the result is returned as an [Expression] of width `1`.
    pub fn sgte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.sgte(&other.0))
    }

    /// [Expression] signed less than. Both [Expression]s must have the same bit width, the result
    /// is returned as an [Expression] of width `1`.
    pub fn slt(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.slt(&other.0))
    }

    /// [Expression] signed less than or equal. Both [Expression]s must have the same bit width,
    /// the result is returned as an [Expression] of width `1`.
    pub fn slte(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.slte(&other.0))
    }

    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.add(&other.0))
    }

    pub fn sub(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.sub(&other.0))
    }

    pub fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.mul(&other.0))
    }

    pub fn udiv(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.udiv(&other.0))
    }

    pub fn sdiv(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.sdiv(&other.0))
    }

    pub fn urem(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.urem(&other.0))
    }

    pub fn srem(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.srem(&other.0))
    }

    pub fn not(&self) -> Self {
        Self(self.0.not())
    }

    pub fn and(&self, other: &Self) -> Self {
        Self(self.0.and(&other.0))
    }

    pub fn or(&self, other: &Self) -> Self {
        Self(self.0.or(&other.0))
    }

    pub fn xor(&self, other: &Self) -> Self {
        Self(self.0.xor(&other.0))
    }

    /// Shift left logical
    pub fn sll(&self, other: &Self) -> Self {
        Self(self.0.sll(&other.0))
    }

    /// Shift right logical
    pub fn srl(&self, other: &Self) -> Self {
        Self(self.0.srl(&other.0))
    }

    /// Shift right arithmetic
    pub fn sra(&self, other: &Self) -> Self {
        Self(self.0.sra(&other.0))
    }

    pub fn ite(&self, then_bv: &Self, else_bv: &Self) -> Self {
        assert_eq!(self.len(), 1);
        Self(self.0.cond_bv(&then_bv.0, &else_bv.0))
    }

    pub fn concat(&self, other: &Self) -> Self {
        Self(self.0.concat(&other.0))
    }

    pub fn slice(&self, low: u32, high: u32) -> Self {
        assert!(low <= high);
        assert!(high <= self.len());
        Self(self.0.slice(high, low))
    }

    pub fn uaddo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.uaddo(&other.0))
    }

    pub fn saddo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.saddo(&other.0))
    }

    pub fn usubo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.usubo(&other.0))
    }

    pub fn ssubo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.ssubo(&other.0))
    }

    pub fn umulo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.umulo(&other.0))
    }

    pub fn smulo(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        Self(self.0.smulo(&other.0))
    }

    pub fn simplify(self) -> Self {
        self
    }

    pub fn get_constant(&self) -> Option<u64> {
        if let Some(binary_str) = self.0.as_binary_str() {
            let num = u64::from_str_radix(&binary_str, 2).unwrap();
            Some(num)
        } else {
            None
        }
    }

    pub fn get_constant_bool(&self) -> Option<bool> {
        assert_eq!(self.len(), 1);
        if let Some(binary_str) = self.0.as_binary_str() {
            Some(binary_str != "0")
        } else {
            None
        }
    }

    pub fn to_binary_string(&self) -> String {
        // TODO: Check if there's a better way to get the an underlying string.
        if self.len() <= 64 {
            let width = self.len() as usize;
            // If we for some reason get less binary digits, pad the start with zeroes.
            format!("{:0width$b}", self.get_constant().unwrap())
        } else {
            let upper = self.slice(64, self.len() - 1).to_binary_string();
            let lower = self.slice(0, 63).to_binary_string();
            format!("{}{}", upper, lower)
        }
    }

    fn get_ctx(&self) -> BoolectorSolverContext {
        let ctx = self.0.get_btor();
        BoolectorSolverContext { ctx }
    }

    pub fn replace_part(&self, start_idx: u32, replace_with: Self) -> Self {
        let end_idx = start_idx + replace_with.len();
        assert!(end_idx <= self.len());

        let value = if start_idx == 0 {
            replace_with
        } else {
            let prefix = self.slice(0, start_idx - 1);
            replace_with.concat(&prefix)
        };

        let value = if end_idx == self.len() {
            value
        } else {
            let suffix = self.slice(end_idx, self.len() - 1);
            suffix.concat(&value)
        };
        assert_eq!(value.len(), self.len());

        value
    }

    /// Saturated unsigned addition. Adds `self` with `other` and if the result overflows the
    /// maximum value is returned.
    ///
    /// Requires that `self` and `other` have the same width.
    pub fn uadds(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());

        let result = self.add(other).simplify();
        let overflow = self.uaddo(other).simplify();
        let saturated = self.get_ctx().unsigned_max(self.len());

        overflow.ite(&saturated, &result)
    }

    /// Saturated signed addition. Adds `self` with `other` and if the result overflows either the
    /// maximum or minimum value is returned, depending on the sign bit of `self`.
    ///
    /// Requires that `self` and `other` have the same width.
    pub fn sadds(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        let width = self.len();

        let result = self.add(other).simplify();
        let overflow = self.saddo(other).simplify();

        let min = self.get_ctx().signed_min(width);
        let max = self.get_ctx().signed_max(width);

        // Check the sign bit if max or min should be given on overflow.
        let is_negative = self.slice(self.len() - 1, self.len() - 1).simplify();

        overflow
            .ite(&is_negative.ite(&min, &max), &result)
            .simplify()
    }

    /// Saturated unsigned subtraction.
    ///
    /// Subtracts `self` with `other` and if the result overflows it is clamped to zero, since the
    /// values are unsigned it can never go below the minimum value.
    pub fn usubs(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());

        let result = self.sub(other).simplify();
        let overflow = self.usubo(other).simplify();

        let zero = self.get_ctx().zero(self.len());
        overflow.ite(&zero, &result)
    }

    /// Saturated signed subtraction.
    ///
    /// Subtracts `self` with `other` with the result clamped between the largest and smallest
    /// value allowed by the bit-width.
    pub fn ssubs(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());

        let result = self.sub(other).simplify();
        let overflow = self.ssubo(other).simplify();

        let width = self.len();
        let min = self.get_ctx().signed_min(width);
        let max = self.get_ctx().signed_max(width);

        // Check the sign bit if max or min should be given on overflow.
        let is_negative = self.slice(self.len() - 1, self.len() - 1).simplify();

        overflow
            .ite(&is_negative.ite(&min, &max), &result)
            .simplify()
    }
}

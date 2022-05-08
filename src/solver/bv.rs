#![allow(clippy::len_without_is_empty)]
use boolector::{BVSolution, Btor};
use std::{cmp::Ordering, rc::Rc};

use crate::Solver;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BV(pub(crate) boolector::BV<Rc<Btor>>);

impl BV {
    /// Returns the bit width of the [BV].
    pub fn len(&self) -> u32 {
        self.0.get_width()
    }

    /// Zero-extend the current [BV] to the passed bit width and return the resulting [BV].
    pub fn zero_ext(&self, width: u32) -> BV {
        match self.len().cmp(&width) {
            Ordering::Less => BV(self.0.uext(width - self.len())),
            Ordering::Equal => self.clone(),
            Ordering::Greater => todo!(),
        }
    }

    /// Sign-extend the current [BV] to the passed bit width and return the resulting [BV].
    pub fn sign_ext(&self, width: u32) -> BV {
        match self.len().cmp(&width) {
            Ordering::Less => BV(self.0.sext(width - self.len())),
            Ordering::Equal => self.clone(),
            Ordering::Greater => todo!(),
        }
    }

    /// Resize the current [BV] to the passed bit width and return the resulting [BV].
    ///
    /// If `self.len()` is compared to `target_size`
    ///
    /// - Same: the symbol is returned.
    /// - Smaller: the symbol is zero extended to the the target size.
    /// - Larger: the symbol is truncated to the target size.
    pub fn resize_unsigned(self, width: u32) -> BV {
        match self.len().cmp(&width) {
            Ordering::Equal => self,
            Ordering::Less => self.zero_ext(width),
            Ordering::Greater => self.slice(0, width - 1),
        }
    }

    /// [BV] equality check. Both [BV]s must have the same bit width, the result is returned as a
    /// [BV] of width `1`.
    pub fn eq(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0._eq(&other.0))
    }

    /// [BV] inequality check. Both [BV]s must have the same bit width, the result is returned as a
    /// [BV] of width `1`.
    pub fn ne(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0._ne(&other.0))
    }

    /// [BV] unsigned greater than. Both [BV]s must have the same bit width, the result is returned
    /// as a [BV] of width `1`.
    pub fn ugt(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.ugt(&other.0))
    }

    /// [BV] unsigned greater than or equal. Both [BV]s must have the same bit width, the result is
    /// returned as a [BV] of width `1`.
    pub fn ugte(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.ugte(&other.0))
    }

    /// [BV] unsigned less than. Both [BV]s must have the same bit width, the result is returned as
    /// a [BV] of width `1`.
    pub fn ult(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.ult(&other.0))
    }

    /// [BV] unsigned less than or equal. Both [BV]s must have the same bit width, the result is
    /// returned as a [BV] of width `1`.
    pub fn ulte(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.ulte(&other.0))
    }

    /// [BV] signed greater than. Both [BV]s must have the same bit width, the result is returned as
    /// a [BV] of width `1`.
    pub fn sgt(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.sgt(&other.0))
    }

    /// [BV] signed greater or equal than. Both [BV]s must have the same bit width, the result is
    /// returned as a [BV] of width `1`.
    pub fn sgte(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.sgte(&other.0))
    }

    /// [BV] signed less than. Both [BV]s must have the same bit width, the result is returned as a
    /// [BV] of width `1`.
    pub fn slt(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.slt(&other.0))
    }

    /// [BV] signed less than or equal. Both [BV]s must have the same bit width, the result is
    /// returned as a [BV] of width `1`.
    pub fn slte(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.slte(&other.0))
    }

    // ---------------------------------------------------------------------------------------------
    // Binary ops
    // ---------------------------------------------------------------------------------------------

    pub fn add(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.add(&other.0))
    }

    pub fn sub(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.sub(&other.0))
    }

    pub fn mul(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.mul(&other.0))
    }

    pub fn udiv(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.udiv(&other.0))
    }

    pub fn sdiv(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.sdiv(&other.0))
    }

    pub fn urem(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.urem(&other.0))
    }

    pub fn srem(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.srem(&other.0))
    }

    // ---------------------------------------------------------------------------------------------
    // Overflowing operations
    // ---------------------------------------------------------------------------------------------

    pub fn uaddo(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.uaddo(&other.0))
    }

    pub fn saddo(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.saddo(&other.0))
    }

    pub fn usubo(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.usubo(&other.0))
    }

    pub fn ssubo(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.ssubo(&other.0))
    }

    pub fn umulo(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.umulo(&other.0))
    }

    pub fn smulo(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.smulo(&other.0))
    }

    // ---------------------------------------------------------------------------------------------
    // Saturating operations
    // ---------------------------------------------------------------------------------------------

    /// Saturated unsigned addition. Adds `self` with `other` and if the result overflows the
    /// maximum value is returned.
    ///
    /// Requires that `self` and `other` have the same width.
    ///
    /// The returned value is a if-then-else BV, which if the result would overflow is the maximum,
    /// otherwise it is the result.
    pub fn uadds(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());

        let result = self.add(other);
        let overflow = self.uaddo(other);
        let saturated = self.get_solver().bv_unsigned_max(self.len());

        overflow.ite(&saturated, &result)
    }

    /// Saturated signed addition. Adds `self` with `other` and if the result overflows either the
    /// maximum or minimum value is returned, depending on the sign bit of `self`.
    ///
    /// Requires that `self` and `other` have the same width.
    ///
    /// The returned value is an if-then-else BV, which returns either the maximum or minimum if the
    /// result would overflow, and the result otherwise.
    pub fn sadds(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        let width = self.len();
        let solver = self.get_solver();

        let result = self.add(other);
        let overflow = self.saddo(other);

        let min = solver.bv_signed_min(width);
        let max = solver.bv_signed_max(width);

        // Check the sign bit.
        let is_negative = self.slice(self.len() - 1, self.len() - 1);

        overflow.ite(&is_negative.ite(&min, &max), &result)
    }

    // ---------------------------------------------------------------------------------------------
    // Logical ops
    // ---------------------------------------------------------------------------------------------

    pub fn not(&self) -> BV {
        BV(self.0.not())
    }

    pub fn and(&self, other: &BV) -> BV {
        BV(self.0.and(&other.0))
    }

    pub fn or(&self, other: &BV) -> BV {
        BV(self.0.or(&other.0))
    }

    pub fn xor(&self, other: &BV) -> BV {
        BV(self.0.xor(&other.0))
    }

    // ---------------------------------------------------------------------------------------------
    // Shifts
    // ---------------------------------------------------------------------------------------------

    /// Shift left logical
    pub fn sll(&self, other: &BV) -> BV {
        BV(self.0.sll(&other.0))
    }

    /// Shift right logical
    pub fn srl(&self, other: &BV) -> BV {
        BV(self.0.srl(&other.0))
    }

    /// Shift right arithmetic
    pub fn sra(&self, other: &BV) -> BV {
        BV(self.0.sra(&other.0))
    }

    // ---------------------------------------------------------------------------------------------
    // Concat
    // ---------------------------------------------------------------------------------------------

    pub fn concat(&self, other: &BV) -> BV {
        BV(self.0.concat(&other.0))
    }

    pub fn slice(&self, low: u32, high: u32) -> BV {
        assert!(low <= high);
        assert!(high <= self.len());
        BV(self.0.slice(high, low))
    }

    /// Replaces part of the BV starting at `start_idx` with `replace_with`
    ///
    /// So for example to replace the BV `10101010` with `1111` starting at index 4, it would result
    /// in `11111010`. Requires that the final length of the BV will stay the same.
    pub fn replace_part(&self, start_idx: u32, replace_with: BV) -> BV {
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

    // ---------------------------------------------------------------------------------------------
    // Conditionals
    // ---------------------------------------------------------------------------------------------

    pub fn ite(&self, then_bv: &BV, else_bv: &BV) -> BV {
        assert_eq!(self.len(), 1);
        BV(self.0.cond_bv(&then_bv.0, &else_bv.0))
    }

    // ---------------------------------------------------------------------------------------------
    // Misc
    // ---------------------------------------------------------------------------------------------

    pub fn get_solver(&self) -> Solver {
        let btor = self.0.get_btor();
        Solver(btor)
    }

    pub(super) fn get_solution(&self) -> BVSolution {
        self.0.get_a_solution()
    }
}

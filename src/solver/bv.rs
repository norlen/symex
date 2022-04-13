use boolector::{BVSolution, Btor};
use std::convert::From;
use std::{ops::Deref, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BV(pub(super) boolector::BV<Rc<Btor>>);

impl BV {
    /// Returns the bidwidth of the [BV].
    pub fn len(&self) -> u32 {
        self.0.get_width()
    }

    /// Zero-extend the current [BV] to the passed bitwidth and return the
    /// resulting [BV].
    pub fn zero_ext(&self, width: u32) -> BV {
        let w = self.0.get_width();

        match w.cmp(&width) {
            std::cmp::Ordering::Less => BV(self.0.uext(width - w)),
            std::cmp::Ordering::Equal => self.clone(),
            std::cmp::Ordering::Greater => todo!(),
        }
    }

    /// Sign-extend the current [BV] to the passed bidwidth and return the
    /// resulting [BV].
    pub fn sign_ext(&self, width: u32) -> BV {
        let w = self.0.get_width();

        match w.cmp(&width) {
            std::cmp::Ordering::Less => BV(self.0.sext(width - w)),
            std::cmp::Ordering::Equal => self.clone(),
            std::cmp::Ordering::Greater => todo!(),
        }
    }

    // -------------------------------------------------------------------------
    // Operations
    // -------------------------------------------------------------------------

    /// [BV] equality check. Both [BV]s must have the same bidwidth, the result
    /// is returned as a [BV] of width 1.
    pub fn eq(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0._eq(&other.0))
    }

    pub fn ne(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0._ne(&other.0))
    }

    pub fn ugt(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.ugt(&other.0))
    }

    pub fn ugte(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.ugte(&other.0))
    }

    pub fn ult(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.ult(&other.0))
    }

    pub fn ulte(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.ulte(&other.0))
    }

    pub fn sgt(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.sgt(&other.0))
    }

    pub fn sgte(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.sgte(&other.0))
    }

    pub fn slt(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.slt(&other.0))
    }

    pub fn slte(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        BV(self.0.slte(&other.0))
    }

    // -------------------------------------------------------------------------
    // Binary ops
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    // Overflowing operations
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    // Saturating operations
    // -------------------------------------------------------------------------

    /// Saturated unsigned addition. Adds `self` with `other` and if the result
    /// overflows the maxmium value is returned.
    ///
    /// Requires that `self` and `other` have the same width.
    ///
    /// The returned value is a if-then-else BV, which if the result would
    /// overflow is the maximum, otherwise it is the result.
    pub fn uadds(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());

        let result = self.add(other);
        let overflow = self.uaddo(other);
        let saturated = BV(boolector::BV::ones(self.0.get_btor(), self.len()));

        overflow.ite(&saturated, &result)
    }

    /// Saturated signed addition. Adds `self` with `other` and if the result
    /// overflows either the maximum or mimimum value is returned, depending
    /// on the sign bit of `self`.
    ///
    /// Requires that `self` and `other` have the same width.
    ///
    /// The returned value is an if-then-else BV, which returns either the
    /// maximum or minimum if the result would overflow, and the result
    /// otherwise.
    pub fn sadds(&self, other: &BV) -> BV {
        assert_eq!(self.len(), other.len());
        let width = self.len();
        let solver = self.0.get_btor();

        let result = self.add(other);
        let overflow = self.saddo(other);

        // Check the sign bit.
        let is_negative = self.slice(self.len() - 1, self.len() - 1);

        // Minimum value: 1000...0
        let min = BV(boolector::BV::one(solver.clone(), 1)
            .concat(&boolector::BV::zero(solver.clone(), width - 1)));

        // Maximum value: 0111...1
        let max = BV(boolector::BV::zero(solver.clone(), 1)
            .concat(&boolector::BV::one(solver.clone(), width - 1)));

        overflow.ite(&is_negative.ite(&min, &max), &result)
    }

    // -------------------------------------------------------------------------
    // Logical ops
    // -------------------------------------------------------------------------

    pub fn and(&self, other: &BV) -> BV {
        BV(self.0.and(&other.0))
    }
    pub fn or(&self, other: &BV) -> BV {
        BV(self.0.or(&other.0))
    }
    pub fn xor(&self, other: &BV) -> BV {
        BV(self.0.xor(&other.0))
    }

    // -------------------------------------------------------------------------
    // Concat
    // -------------------------------------------------------------------------

    pub fn concat(&self, other: &BV) -> BV {
        BV(self.0.concat(&other.0))
    }

    pub fn slice(&self, low: u32, high: u32) -> BV {
        assert!(low <= high);
        assert!(high <= self.len());
        BV(self.0.slice(high, low))
    }

    // -------------------------------------------------------------------------
    // Conditionals
    // -------------------------------------------------------------------------

    pub fn ite(&self, then_bv: &BV, else_bv: &BV) -> BV {
        assert_eq!(self.len(), 1);
        BV(self.0.cond_bv(&then_bv.0, &else_bv.0))
    }

    // -------------------------------------------------------------------------
    // Solutions
    // -------------------------------------------------------------------------

    pub fn get_solution(&self) -> BVSolution {
        self.0.get_a_solution()
    }

    // -------------------------------------------------------------------------
    // Deprecated
    // -------------------------------------------------------------------------

    #[deprecated]
    pub fn _ne(&self, other: &BV) -> BV {
        self.0._ne(&other.0).into()
    }

    #[deprecated]
    pub fn _eq(&self, other: &BV) -> BV {
        self.0._eq(&other.0).into()
    }

    #[deprecated]
    pub fn get_width(&self) -> u32 {
        self.0.get_width()
    }

    #[deprecated]
    pub fn assert(&self) {
        self.0.assert();
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

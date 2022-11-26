use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolverError {
    /// The set of constraints added to the solution are unsatisfiable.
    #[error("Unsat")]
    Unsat,

    /// Unknown error passed along from the SMT solver used.
    #[error("Unknown")]
    Unknown,

    /// Exceeded the passed maximum number of solutions.
    #[error("Exceeded number of solutions")]
    TooManySolutions,
}

#[derive(Debug)]
pub enum Solutions<E> {
    Exactly(Vec<E>),
    AtLeast(Vec<E>),
}

/// SolverContext handles the creation of expressions.
///
/// Keeps track of all the created expressions and the internal SMT state.
pub trait SolverContext<E: Expression> {
    /// Associated solver to be used with the context.
    type Solver: Solver;

    /// Create a new uninitialized expression of size `bits`.
    fn unconstrained(&self, bits: u32, name: &str) -> E;

    /// Create a new expression set equal to `1` of size `bits.
    fn one(&self, bits: u32) -> E;

    /// Create a new expression set to zero of size `bits.
    fn zero(&self, bits: u32) -> E;

    /// Create a new expression from a boolean value.
    fn from_bool(&self, value: bool) -> E;

    /// Create a new expression from an `u64` value of size `bits`.
    fn from_u64(&self, value: u64, bits: u32) -> E;

    /// Create an expression of size `bits` from a binary string.
    fn from_binary_string(&self, bits: &str) -> E;

    /// Creates an expression of size `bits` containing the maximum unsigned value.
    fn unsigned_max(&self, bits: u32) -> E {
        let mut s = String::new();
        s.reserve_exact(bits as usize);
        for _ in 0..bits {
            s.push('1');
        }
        self.from_binary_string(&s)
    }

    /// Create an expression of size `bits` containing the maximum signed value.
    fn signed_max(&self, bits: u32) -> E {
        // Maximum value: 0111...1
        assert!(bits > 1);
        let mut s = String::from("0");
        s.reserve_exact(bits as usize);
        for _ in 0..bits - 1 {
            s.push('1');
        }
        self.from_binary_string(&s)
    }

    /// Create an expression of size `bits` containing the minimum signed value.
    fn signed_min(&self, bits: u32) -> E {
        // Minimum value: 1000...0
        assert!(bits > 1);
        let mut s = String::from("1");
        s.reserve_exact(bits as usize);
        for _ in 0..bits - 1 {
            s.push('0');
        }
        self.from_binary_string(&s)
    }
}

/// Symbolic array where both index and stored values are symbolic.
pub trait Array: Sized + Clone + Debug {
    /// Expressions used for index and stored in the array.
    type Expression: Expression;

    /// Associated context used for creation of the array, and stored expressions.
    type Context: SolverContext<Self::Expression>;

    /// Create a new array where index has size `index_size` and each element has size `element_size`.
    fn new(ctx: &Self::Context, index_size: usize, element_size: usize, name: &str) -> Self;

    /// Return value with specific index.
    fn read(&self, index: &Self::Expression) -> Self::Expression;

    /// Write value to index.
    fn write(&mut self, index: &Self::Expression, value: Self::Expression);
}

pub trait Expression: Sized + Clone + Debug {
    type Context: SolverContext<Self>;

    /// Returns the bit width of the [Expression].
    fn len(&self) -> u32;

    /// Zero-extend the current [Expression] to the passed bit width and return the resulting
    /// [Expression].
    fn zero_ext(&self, width: u32) -> Self;

    /// Sign-extend the current [Expression] to the passed bit width and return the resulting
    /// [Expression].
    fn sign_ext(&self, width: u32) -> Self;

    fn resize_unsigned(&self, width: u32) -> Self;

    /// [Expression] equality check. Both [Expression]s must have the same bit width, the result is
    /// returned as an [Expression] of width `1`.
    fn _eq(&self, other: &Self) -> Self;

    /// [Expression] inequality check. Both [Expression]s must have the same bit width, the result is
    /// returned as an [Expression] of width `1`.
    fn _ne(&self, other: &Self) -> Self;

    /// [Expression] unsigned greater than. Both [Expression]s must have the same bit width, the
    /// result is returned as an [Expression] of width `1`.
    fn ugt(&self, other: &Self) -> Self;

    /// [Expression] unsigned greater than or equal. Both [Expression]s must have the same bit
    /// width, the result is returned as an [Expression] of width `1`.
    fn ugte(&self, other: &Self) -> Self;

    /// [Expression] unsigned less than. Both [Expression]s must have the same bit width, the result
    /// is returned as an [Expression] of width `1`.
    fn ult(&self, other: &Self) -> Self;

    /// [Expression] unsigned less than or equal. Both [Expression]s must have the same bit width,
    /// the result is returned as an [Expression] of width `1`.
    fn ulte(&self, other: &Self) -> Self;

    /// [Expression] signed greater than. Both [Expression]s must have the same bit width, the
    /// result is returned as an [Expression] of width `1`.
    fn sgt(&self, other: &Self) -> Self;

    /// [Expression] signed greater or equal than. Both [Expression]s must have the same bit width,
    /// the result is returned as an [Expression] of width `1`.
    fn sgte(&self, other: &Self) -> Self;

    /// [Expression] signed less than. Both [Expression]s must have the same bit width, the result
    /// is returned as an [Expression] of width `1`.
    fn slt(&self, other: &Self) -> Self;

    /// [Expression] signed less than or equal. Both [Expression]s must have the same bit width,
    /// the result is returned as an [Expression] of width `1`.
    fn slte(&self, other: &Self) -> Self;

    fn add(&self, other: &Self) -> Self;

    fn sub(&self, other: &Self) -> Self;

    fn mul(&self, other: &Self) -> Self;

    fn udiv(&self, other: &Self) -> Self;

    fn sdiv(&self, other: &Self) -> Self;

    fn urem(&self, other: &Self) -> Self;

    fn srem(&self, other: &Self) -> Self;

    fn not(&self) -> Self;

    fn and(&self, other: &Self) -> Self;

    fn or(&self, other: &Self) -> Self;

    fn xor(&self, other: &Self) -> Self;

    /// Shift left logical
    fn sll(&self, other: &Self) -> Self;

    /// Shift right logical
    fn srl(&self, other: &Self) -> Self;

    /// Shift right arithmetic
    fn sra(&self, other: &Self) -> Self;

    fn ite(&self, then_bv: &Self, else_bv: &Self) -> Self;

    fn concat(&self, other: &Self) -> Self;

    fn slice(&self, low: u32, high: u32) -> Self;

    fn replace_part(&self, start_idx: u32, replace_with: Self) -> Self {
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

    fn uaddo(&self, other: &Self) -> Self;

    fn saddo(&self, other: &Self) -> Self;

    fn usubo(&self, other: &Self) -> Self;

    fn ssubo(&self, other: &Self) -> Self;

    fn umulo(&self, other: &Self) -> Self;

    fn smulo(&self, other: &Self) -> Self;

    /// Saturated unsigned addition. Adds `self` with `other` and if the result overflows the
    /// maximum value is returned.
    ///
    /// Requires that `self` and `other` have the same width.
    fn uadds(&self, other: &Self) -> Self {
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
    fn sadds(&self, other: &Self) -> Self {
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
    fn usubs(&self, other: &Self) -> Self {
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
    fn ssubs(&self, other: &Self) -> Self {
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

    fn simplify(self) -> Self;

    fn get_constant(&self) -> Option<u64>;

    fn get_constant_bool(&self) -> Option<bool>;

    fn to_binary_string(&self) -> String;

    fn get_ctx(&self) -> Self::Context;
}

pub trait Solver: Debug {
    type E: Expression;

    /// Solve for the current solver state, and returns if the result is satisfiable.
    ///
    /// All asserts and assumes are implicitly combined with a boolean and. Returns true or false,
    /// and [SolverError::Unknown] if the result cannot be determined.
    fn is_sat(&self) -> Result<bool, SolverError>;

    /// Solve for the solver state with the assumption of the passed constraint.
    fn is_sat_with_constraint(&self, constraint: &Self::E) -> Result<bool, SolverError>;

    /// Solve for the solver state with the assumption of the passed constraints.
    fn is_sat_with_constraints(&self, constraints: &[Self::E]) -> Result<bool, SolverError>;

    /// Add the constraint to the solver.
    ///
    /// The passed constraint will be implicitly combined with the current state in a boolean `and`.
    /// Asserted constraints cannot be removed.
    fn assert(&self, constraint: &Self::E);

    /// Returns `true` if `lhs` and `rhs` must be equal under the current constraints.
    fn must_be_equal(&self, lhs: &Self::E, rhs: &Self::E) -> Result<bool, SolverError> {
        // Add the constraint lhs != rhs and invert the results. The only way
        // for `lhs != rhs` to be `false` is that if they are equal.
        let constraint = lhs._ne(rhs);
        let result = self.is_sat_with_constraint(&constraint)?;
        Ok(!result)
    }

    /// Check if `lhs` and `rhs` can be equal under the current constraints.
    fn can_equal(&self, lhs: &Self::E, rhs: &Self::E) -> Result<bool, SolverError> {
        self.is_sat_with_constraint(&lhs._eq(rhs))
    }

    /// Find solutions to `expr`.
    ///
    /// Returns concrete solutions up to `upper_bound`, the returned [`Solutions`] has variants
    /// for if the number of solution exceeds the upper bound.
    fn get_values(
        &self,
        expr: &Self::E,
        upper_bound: usize,
    ) -> Result<Solutions<Self::E>, SolverError>;

    /// Find solutions to `expr`.
    ///
    /// Returns concrete solutions up to a maximum of `upper_bound`. If more solutions are available
    /// the error [`SolverError::TooManySolutions`] is returned.
    fn get_solutions(
        &self,
        expr: &Self::E,
        upper_bound: usize,
    ) -> Result<Vec<Self::E>, SolverError>;
}

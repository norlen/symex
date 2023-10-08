use boolector::{
    option::{BtorOption, NumberFormat},
    Btor, BV,
};
use std::rc::Rc;

mod expr;
mod solver;

// Re-exports.
pub(super) use expr::BoolectorExpr;
pub(super) use solver::BoolectorIncrementalSolver;

/// SolverContext handles the creation of expressions.
///
/// Keeps track of all the created expressions and the internal SMT state.
#[derive(Debug, Clone)]
pub struct BoolectorSolverContext {
    pub ctx: Rc<Btor>,
}

impl BoolectorSolverContext {
    /// Create a new uninitialized expression of size `bits`.
    pub fn unconstrained(&self, bits: u32, name: &str) -> BoolectorExpr {
        BoolectorExpr(BV::new(self.ctx.clone(), bits, Some(name)))
    }

    /// Create a new expression set equal to `1` of size `bits.
    pub fn one(&self, bits: u32) -> BoolectorExpr {
        BoolectorExpr(boolector::BV::from_u64(self.ctx.clone(), 1, bits))
    }

    /// Create a new expression set to zero of size `bits.
    pub fn zero(&self, bits: u32) -> BoolectorExpr {
        BoolectorExpr(boolector::BV::zero(self.ctx.clone(), bits))
    }

    /// Create a new expression from a boolean value.
    pub fn from_bool(&self, value: bool) -> BoolectorExpr {
        BoolectorExpr(boolector::BV::from_bool(self.ctx.clone(), value))
    }

    /// Create a new expression from an `u64` value of size `bits`.
    pub fn from_u64(&self, value: u64, bits: u32) -> BoolectorExpr {
        BoolectorExpr(boolector::BV::from_u64(self.ctx.clone(), value, bits))
    }

    /// Create an expression of size `bits` from a binary string.
    pub fn from_binary_string(&self, bits: &str) -> BoolectorExpr {
        BoolectorExpr(boolector::BV::from_binary_str(self.ctx.clone(), bits))
    }

    /// Creates an expression of size `bits` containing the maximum unsigned value.
    pub fn unsigned_max(&self, bits: u32) -> BoolectorExpr {
        let mut s = String::new();
        s.reserve_exact(bits as usize);
        for _ in 0..bits {
            s.push('1');
        }
        self.from_binary_string(&s)
    }

    /// Create an expression of size `bits` containing the maximum signed value.
    pub fn signed_max(&self, bits: u32) -> BoolectorExpr {
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
    pub fn signed_min(&self, bits: u32) -> BoolectorExpr {
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

impl BoolectorSolverContext {
    pub fn new() -> Self {
        let btor = Btor::new();
        let ctx = Rc::new(btor);
        ctx.set_opt(BtorOption::Incremental(true));
        ctx.set_opt(BtorOption::PrettyPrint(true));
        ctx.set_opt(BtorOption::OutputNumberFormat(NumberFormat::Hexadecimal));

        Self { ctx }
    }
}

/// Symbolic array where both index and stored values are symbolic.
#[derive(Debug, Clone)]
pub struct BoolectorArray(pub(super) boolector::Array<Rc<Btor>>);

impl BoolectorArray {
    /// Create a new array where index has size `index_size` and each element has size `element_size`.
    pub fn new(
        ctx: &BoolectorSolverContext,
        index_size: usize,
        element_size: usize,
        name: &str,
    ) -> Self {
        let memory = boolector::Array::new(
            ctx.ctx.clone(),
            index_size as u32,
            element_size as u32,
            Some(name),
        );

        Self(memory)
    }

    /// Return value with specific index.
    pub fn read(&self, index: &BoolectorExpr) -> BoolectorExpr {
        BoolectorExpr(self.0.read(&index.0))
    }

    /// Write value to index.
    pub fn write(&mut self, index: &BoolectorExpr, value: BoolectorExpr) {
        self.0 = self.0.write(&index.0, &value.0)
    }
}

use boolector::{
    option::{BtorOption, ModelGen, NumberFormat},
    BVSolution, Btor, SolverResult,
};
use std::{collections::HashMap, rc::Rc};
use thiserror::Error;

mod array;
mod bv;

pub use self::{array::Array, bv::BV};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolverError {
    #[error("Solver state unknown")]
    Unknown,
}

#[derive(Debug)]
pub enum Solutions {
    /// Could not find any solutions with the current constraints.
    None,

    /// Found these solutions, and no more.
    Exactly(Vec<BVSolution>),

    /// Non-exhaustive list of solutions, there exist more than this.
    AtLeast(Vec<BVSolution>),
}

/// Helper used to generate solutions for symbols.
///
/// Solutions can be generated using methods on the solver. Using this instead provides a cache
/// so it multiple bindings map to the same symbol only one solution has to be generated.
pub struct SolutionGenerator {
    solver: Solver,

    cache: HashMap<i32, BVSolution>,
}

impl Drop for SolutionGenerator {
    fn drop(&mut self) {
        self.solver
            .0
            .set_opt(BtorOption::ModelGen(ModelGen::Disabled));
        self.solver.pop();
    }
}

impl SolutionGenerator {
    pub fn new(solver: Solver) -> Self {
        solver.push();
        solver.0.set_opt(BtorOption::ModelGen(ModelGen::All));

        Self {
            solver,
            cache: HashMap::new(),
        }
    }

    pub fn get_solution(&mut self, bv: &BV) -> Result<BVSolution, SolverError> {
        let id = bv.0.get_id();
        if let Some(cached) = self.cache.get(&id).cloned() {
            return Ok(cached);
        }

        // Setup before checking for solutions.
        if !self.solver.is_sat()? {}

        let solution = bv.get_solution().disambiguate();
        self.cache.insert(id, solution.clone());
        Ok(solution)
    }
}

#[derive(Debug, Clone)]
pub struct Solver(pub(crate) Rc<Btor>);

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    pub fn new() -> Self {
        let btor = Btor::new();
        btor.set_opt(BtorOption::Incremental(true));
        btor.set_opt(BtorOption::PrettyPrint(true));
        btor.set_opt(BtorOption::OutputNumberFormat(NumberFormat::Hexadecimal));

        Self(Rc::new(btor))
    }

    /// A regular clone will only clone the reference, a duplication will instead create a new
    /// instance of the underlying solver.
    pub fn duplicate(&self) -> Self {
        Self(Rc::new(self.0.duplicate()))
    }

    /// Solve for the current solver state, and returns if the result is satisfiable.
    ///
    /// All asserts and assumes are implicitly combined with a boolean and. Returns true or false,
    /// and [SolverError::Unknown] if the result cannot be determined.
    pub fn is_sat(&self) -> Result<bool, SolverError> {
        match self.0.sat() {
            SolverResult::Sat => Ok(true),
            SolverResult::Unsat => Ok(false),
            SolverResult::Unknown => Err(SolverError::Unknown),
        }
    }

    /// Solve for the solver state with the passed constraint asserted.
    pub fn is_sat_with_constraint(&self, constraint: &BV) -> Result<bool, SolverError> {
        // Assume the constraint, will be forgotten after the next call to `is_sat`.
        constraint.0.assume();
        self.is_sat()
    }

    /// Add the constraint to the solver.
    ///
    /// The passed constraint will be implicitly combined with the current state in a boolean `and`.
    /// Asserted constraints cannot be removed.
    pub fn assert(&self, bv: &BV) {
        bv.0.assert();
    }

    /// Returns `true` if `lhs` and `rhs` must be equal under the current constraints.
    pub fn must_be_equal(&self, lhs: &BV, rhs: &BV) -> Result<bool, SolverError> {
        // Add the constraint lhs != rhs and invert the results. The only way
        // for `lhs != rhs` to be `false` is that if they are equal.
        let constraint = lhs.ne(rhs);
        let result = self.is_sat_with_constraint(&constraint)?;
        Ok(!result)
    }

    /// Check if `lhs` and `rhs` can be equal under the current constraints.
    pub fn can_equal(&self, lhs: &BV, rhs: &BV) -> Result<bool, SolverError> {
        self.is_sat_with_constraint(&lhs.eq(rhs))
    }

    pub fn get_solutions_for_bv(
        &self,
        bv: &BV,
        max_solutions: usize,
    ) -> Result<Solutions, SolverError> {
        // Setup before checking for solutions.
        self.push();
        self.0.set_opt(BtorOption::ModelGen(ModelGen::All));

        let result = self.internal_get_solutions_for_bv(bv, max_solutions);

        // Restore solver to initial state.
        self.0.set_opt(BtorOption::ModelGen(ModelGen::Disabled));
        self.pop();

        result
    }

    /// Add a context level to the solver.
    ///
    /// Adding a context level to the solver allows for adding constraints that can be forgotten
    /// later with a call to `pop`.
    pub fn push(&self) {
        self.0.push(1)
    }

    /// Remove a context level from the solver.
    ///
    /// Removing a context level will remove all asserted constraints from that level.
    pub fn pop(&self) {
        self.0.pop(1)
    }

    /// Create a new uninitialized bitvector array.
    ///
    /// The array will be indexed with bitvectors of size `index_width`, and each element in the
    /// array has the size `element_width`.
    pub fn array(&self, index_width: u32, element_width: u32, symbol: Option<&str>) -> Array {
        Array(boolector::Array::new(
            self.0.clone(),
            index_width,
            element_width,
            symbol,
        ))
    }

    /// Create a new uninitialized bitvector of size `bits`.
    pub fn bv(&self, bits: u32, name: &str) -> BV {
        BV(boolector::BV::new(self.0.clone(), bits, Some(name)))
    }

    /// Create a new unnamed uninitialized bitvector of size `bits`.
    pub fn bv_unnamed(&self, bits: u32) -> BV {
        BV(boolector::BV::new(self.0.clone(), bits, None))
    }

    /// Create a new symbol from a boolean value.
    pub fn bv_from_bool(&self, value: bool) -> BV {
        BV(boolector::BV::from_bool(self.0.clone(), value))
    }

    /// Create a new symbol from an `u64` value of size `bits`.
    pub fn bv_from_u64(&self, value: u64, bits: u32) -> BV {
        BV(boolector::BV::from_u64(self.0.clone(), value, bits))
    }

    /// Create a new symbol set to zero of size `bits.
    pub fn bv_zero(&self, bits: u32) -> BV {
        BV(boolector::BV::zero(self.0.clone(), bits))
    }

    /// Create a bitvector of size `bits` from a binary string.
    pub fn from_binary_string(&self, bits: &str) -> BV {
        BV(boolector::BV::from_binary_str(self.0.clone(), bits))
    }

    /// Helper to ensure we always set `ModelGen::Disabled` for all paths in this function.
    fn internal_get_solutions_for_bv(
        &self,
        bv: &BV,
        max_solutions: usize,
    ) -> Result<Solutions, SolverError> {
        if !self.is_sat()? {
            return Ok(Solutions::None);
        }

        let mut solutions = Vec::new();
        while solutions.len() < max_solutions && self.is_sat()? {
            let solution = bv.get_solution().disambiguate();

            // Constrain the next value to not be an already found solution.
            let solution_bv = self.from_binary_string(solution.as_01x_str());
            self.assert(&bv.ne(&solution_bv));

            solutions.push(solution);
        }

        if solutions.is_empty() {
            Ok(Solutions::None)
        } else {
            let exists_more_solutions = self.is_sat()?;
            match exists_more_solutions {
                false => Ok(Solutions::Exactly(solutions)),
                true => Ok(Solutions::AtLeast(solutions)),
            }
        }
    }
}

use boolector::{
    option::{BtorOption, ModelGen, NumberFormat, RewriteLevel},
    BVSolution, Btor, SolverResult,
};
use std::{collections::HashMap, rc::Rc};
use thiserror::Error;

mod array;
mod bv;

use crate::VMError;

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
    pub fn new(solver: Solver) -> Result<Self, VMError> {
        if !solver.is_sat()? {
            return Err(VMError::Unsat);
        }

        solver.push();
        solver.0.set_opt(BtorOption::ModelGen(ModelGen::All));

        Ok(Self {
            solver,
            cache: HashMap::new(),
        })
    }

    pub fn get_solution(&mut self, bv: &BV) -> Result<BVSolution, VMError> {
        let id = bv.0.get_id();
        if let Some(cached) = self.cache.get(&id).cloned() {
            return Ok(cached);
        }

        // Setup before checking for solutions.
        if !self.solver.is_sat()? {
            return Err(VMError::Unsat);
        }

        if let Some(binary_str) = bv.0.as_binary_str() {
            Ok(BVSolution::from_01x_str(binary_str))
        } else {
            let solution = bv.get_solution().disambiguate();
            self.cache.insert(id, solution.clone());
            Ok(solution)
        }
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
        btor.set_opt(BtorOption::RewriteLevel(RewriteLevel::Full));
        btor.set_opt(BtorOption::SkeletonPreproc(true));
        btor.set_opt(BtorOption::Ackermann(true));
        btor.set_opt(BtorOption::BetaReduce(true));
        btor.set_opt(BtorOption::EliminateSlices(true));
        btor.set_opt(BtorOption::VariableSubst(true));
        //btor.set_opt(BtorOption::UnconstrainedOpt(true));
        btor.set_opt(BtorOption::MergeLambdas(true));
        btor.set_opt(BtorOption::ExtractLambdas(true));
        btor.set_opt(BtorOption::Normalize(true));
        btor.set_opt(BtorOption::NormalizeAdd(true));

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
        let is_sat = self.0.sat();
        match is_sat {
            SolverResult::Sat => Ok(true),
            SolverResult::Unsat => Ok(false),
            SolverResult::Unknown => Err(SolverError::Unknown),
        }
    }

    /// Solve for the solver state with the assumption of the passed constraint.
    pub fn is_sat_with_constraint(&self, constraint: &BV) -> Result<bool, SolverError> {
        // Assume the constraint, will be forgotten after the next call to `is_sat`.
        constraint.0.assume();
        self.is_sat()
    }

    /// Solve for the solver state with the assumption of the passed constraints.
    pub fn is_sat_with_constraints(&self, constraints: &[&BV]) -> Result<bool, SolverError> {
        for constraint in constraints {
            constraint.0.assume();
        }
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
        // Check if bit-vector is a constant.
        if let Some(str) = bv.0.as_binary_str() {
            let solution = BVSolution::from_01x_str(str);
            return Ok(Solutions::Exactly(vec![solution]));
        }

        // Setup before checking for solutions.
        self.push();
        self.0.set_opt(BtorOption::ModelGen(ModelGen::All));

        let result = self.internal_get_solutions_for_bv(bv, max_solutions);

        // Restore solver to initial state.
        self.0.set_opt(BtorOption::ModelGen(ModelGen::Disabled));
        self.pop();

        result
    }

    /// Returns the highest value a solution can have for the given bit-vector.
    pub fn get_solution_maximum(&self, bv: &BV) -> Result<u64, SolverError> {
        // Check if bit-vector is a constant.
        if let Some(str) = bv.0.as_binary_str() {
            let solution = u64::from_str_radix(&str, 2).unwrap();
            return Ok(solution);
        }

        self.push();
        self.0.set_opt(BtorOption::ModelGen(ModelGen::All));

        let result = self.internal_get_max_solution(bv);

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

    /// Creates a big-vector of size `bits` containing the maximum unsigned value.
    pub fn bv_unsigned_max(&self, bits: u32) -> BV {
        BV(boolector::BV::ones(self.0.clone(), bits))
    }

    /// Create a bit-vector of size `bits` containing the maximum signed value.
    pub fn bv_signed_max(&self, bits: u32) -> BV {
        assert!(bits > 1);

        // Maximum value: 0111...1
        let leading_zero = boolector::BV::zero(self.0.clone(), 1);
        let ones = boolector::BV::ones(self.0.clone(), bits - 1);
        BV(leading_zero.concat(&ones))
    }

    /// Create a bit-vector of size `bits` containing the minimum signed value.
    pub fn bv_signed_min(&self, bits: u32) -> BV {
        assert!(bits > 1);

        // Minimum value: 1000...0
        let leading_one = boolector::BV::one(self.0.clone(), 1);
        let zeroes = boolector::BV::zero(self.0.clone(), bits - 1);
        BV(leading_one.concat(&zeroes))
    }

    /// Helper to ensure we always set `ModelGen::Disabled` for all paths in this function.
    fn internal_get_solutions_for_bv(
        &self,
        bv: &BV,
        max_solutions: usize,
    ) -> Result<Solutions, SolverError> {
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

    /// Helper to ensure we always set `ModelGen::Disabled` for all paths in this function.
    fn internal_get_max_solution(&self, bv: &BV) -> Result<u64, SolverError> {
        let mut highest = None;
        while self.is_sat()? {
            let solution = bv.get_solution();
            let solution = solution.disambiguate();

            // Constrain the next value to not be an already found solution.
            let solution_bv = self.from_binary_string(solution.as_01x_str());
            highest = Some(solution);

            self.assert(&bv.ugt(&solution_bv));
        }

        match highest {
            // None => Ok(Solutions::None),
            None => panic!("unsat"),
            Some(value) => Ok(value.as_u64().unwrap()),
            // Some(value) => Ok(Solutions::Exactly(vec![value])),
        }
    }
}

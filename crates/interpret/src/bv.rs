// use anyhow::{anyhow, Result};
// use boolector::{
//     option::{BtorOption, ModelGen, NumberFormat},
//     BVSolution, Btor, SolverResult,
// };
// use std::{ops::Deref, rc::Rc};

// pub type BV = boolector::BV<Rc<Btor>>;

// pub fn zero_ext(bv: BV, width: u32) -> BV {
//     match bv.get_width().cmp(&width) {
//         std::cmp::Ordering::Less => bv.uext(width - bv.get_width()),
//         std::cmp::Ordering::Equal => bv,
//         std::cmp::Ordering::Greater => todo!(),
//     }
// }

// pub enum BinaryOperation {
//     Add,
//     Sub,
//     Mul,
//     UDiv,
//     SDiv,
//     URem,
//     SRem,
// }

// pub enum Solutions {
//     /// Could not find any solutions with the current constraints.
//     None,

//     /// Found these solutions, and no more.
//     Exactly(Vec<BVSolution>),

//     /// Non-exhaustive list of solutions, there exist more than this.
//     AtLeast(Vec<BVSolution>),
// }

// #[derive(Debug, Clone)]
// pub struct Solver {
//     btor: Rc<Btor>,
// }

// impl Solver {
//     pub fn new() -> Self {
//         let btor = Btor::new();
//         btor.set_opt(BtorOption::Incremental(true));
//         btor.set_opt(BtorOption::PrettyPrint(true));
//         btor.set_opt(BtorOption::OutputNumberFormat(NumberFormat::Hexadecimal));

//         Self {
//             btor: Rc::new(btor),
//         }
//     }

//     /// Check if it is satisfiable.
//     pub fn is_sat(&self) -> Result<bool> {
//         match self.sat() {
//             SolverResult::Sat => Ok(true),
//             SolverResult::Unsat => Ok(false),
//             SolverResult::Unknown => Err(anyhow!("solver result unknown")),
//         }
//     }

//     /// Check if sat with the extra constraint.
//     pub fn is_sat_with_constraint(&self, constraint: &BV) -> Result<bool> {
//         // Can't i use assume here?
//         // it should only be valid until the next call to is_sat

//         self.push(1);
//         constraint.assert();
//         let is_sat = self.is_sat()?;
//         self.pop(1);
//         Ok(is_sat)
//     }

//     // -------------------------------------------------------------------------
//     // BV helpers
//     // -------------------------------------------------------------------------

//     // /// Returns `true` if `lhs` and `rhs` must be equal under the current
//     // /// constraints.
//     // pub fn bvs_must_be_equal(&self, lhs: &BV, rhs: &BV) -> Result<bool> {
//     //     // Add the constraint lhs != rhs and invert the results. The only way
//     //     // for `lhs != rhs` to be `false` is that if they are equal.
//     //     let constraint = lhs._ne(&rhs);
//     //     let result = self.is_sat_with_constraint(&constraint)?;
//     //     Ok(!result)
//     // }

//     // /// Returns `true` if `lhs` and `rhs` can be equal under the current
//     // /// constraints.
//     // pub fn bvs_can_be_equal(&self, lhs: &BV, rhs: &BV) -> Result<bool> {
//     //     let constraint = lhs._eq(rhs);
//     //     self.is_sat_with_constraint(&constraint)
//     // }

//     pub fn get_solutions_for_bv(&self, bv: &BV, max_solutions: usize) -> Result<Solutions> {
//         // Setup before checking for solutions.
//         self.btor.push(1);
//         self.btor.set_opt(BtorOption::ModelGen(ModelGen::All));

//         let result = self.internal_get_solutions_for_bv(bv, max_solutions);

//         // Restore solver to initial state.
//         self.btor.set_opt(BtorOption::ModelGen(ModelGen::Disabled));
//         self.btor.pop(1);

//         // Return our result from the helper, either an Ok or Err.
//         result
//     }

//     /// Helper to ensure we always set `ModelGen::Disabled` for all paths in this function.
//     fn internal_get_solutions_for_bv(&self, bv: &BV, max_solutions: usize) -> Result<Solutions> {
//         if !self.is_sat()? {
//             return Ok(Solutions::None);
//         }

//         let mut solutions = Vec::new();
//         while solutions.len() <= max_solutions && self.is_sat()? {
//             let solution = bv.get_a_solution().disambiguate();

//             // Constrain the next value to not be an already found solution.
//             let solution_bv = self.from_binary_string(solution.as_01x_str());
//             bv._ne(&solution_bv);

//             solutions.push(solution);
//         }

//         Ok(match solutions.len() {
//             0 => Solutions::None,
//             n if n > max_solutions => Solutions::AtLeast(solutions),
//             _ => Solutions::Exactly(solutions),
//         })
//     }

//     // -------------------------------------------------------------------------
//     // BV creation
//     // -------------------------------------------------------------------------

//     pub fn bv_from_bool(&self, value: bool) -> BV {
//         BV::from_bool(self.btor.clone(), value)
//     }

//     /// Creates new bv from a u64.
//     pub fn bv_from_u64(&self, value: u64, bits: u32) -> BV {
//         BV::from_u64(self.btor.clone(), value, bits)
//     }

//     /// Gets a new bv with a zero value.
//     pub fn bv_zero(&self, bits: u32) -> BV {
//         BV::zero(self.btor.clone(), bits)
//     }

//     pub fn from_binary_string(&self, bits: &str) -> BV {
//         BV::from_binary_str(self.btor.clone(), bits)
//     }

//     // -------------------------------------------------------------------------
//     // todo
//     // -------------------------------------------------------------------------

//     // TODO: Remove.
//     pub fn btor(&self) -> Rc<Btor> {
//         self.btor.clone()
//     }
// }

// impl Deref for Solver {
//     type Target = Rc<Btor>;

//     fn deref(&self) -> &Self::Target {
//         &self.btor
//     }
// }

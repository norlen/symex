//! This library provides a symbolic execution engine that operates on LLVM IR.
//!
//! It aims to be a general purpose execution engine that can analyze mainly Rust code, but should
//! handle other languages that can generate LLVM bitcode.
//!
//! It currently supports LLVM 13, and uses the [boolector](https://crates.io/crates/boolector)
//! library as the SMT solver.
//!
//! # Example
//!
//! Assuming the code to analyze is
//!
//! ```rust
//! fn function_to_analyze(x: u32) -> u32 {
//!     if x < 10 {
//!         x
//!     } else {
//!         10
//!     }
//! }
//! ```
//!
//! Code to analyze the function can be written as
//!
//! ```ignore
//! use x0001e::{Project, ReturnValue, VM};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let project = Project::from_bc_path("path/to/ir.bc")?;
//!     let mut vm = VM::new("function_to_analyze", &project)?;
//!
//!     while let Some(path_result) = vm.run() {
//!         // Result of running one path is contained in the path_result.
//!         //
//!         // The VM state is currently at the terminating instructions, and methods are available
//!         // on the VM to inspect the state and variables.
//!         //
//!         // Most interesting are most likely `vm.parameters` for all input variables to the
//!         // function, and `vm.state.symbolic` for variables marked with `symbolic()`.
//!         let output = match &path_result {
//!             Ok(value) => match value {
//!                 ReturnValue::Value(value) => Some(
//!                     vm.solver.get_solutions_for_bv(value, 1)?, // Get one solution for output.
//!                 ),
//!                 ReturnValue::Void => None,
//!             },
//!             Err(_) => None,
//!         };
//!
//!         let inputs: Result<Vec<_>, _> = vm
//!             .parameters
//!             .iter()
//!             .map(|input| vm.solver.get_solutions_for_bv(input, 1))
//!             .collect();
//!
//!         println!("Inputs: {inputs:?}");
//!         println!("Output: {output:?}");
//!     }
//!     Ok(())
//! }
//! ```
//!
//! This should give two possible paths through the program. For the condition `x < 10` then one
//! for the then branch, with the constraint that `x < 10` and one for the else branch with the
//! constraint `x >= 10`.
//!
//!
//! # Details
//!
//! ## Project
//!
//! A [Project] is a collection of LLVM IR modules. It holds the IR code and can be queried for size
//! information about each type, which functions are available. Custom hooks and intrinsics are
//! also present in the project.
//!
//! ## VM
//!
//! A [VM] is created from a project and a function name. A single VM holds mutable state for a
//! run of that function. Multiple VMs can be spawned from a single project, for different
//! functions.
//!
//! The mutable state it holds are memory, allocated variables, the callstack, and so on. This state
//! is cloned for each path through the given function.
//!
//! ## Solver
//!
//! The [Solver] keeps tracks of all the contraints added to it. It it also used to create new
//! symbols. The solver can be queried for solutions to different symbols.
//!
//! ## Hooks
//!
//! Hooks can be added for any function the IR calls. They are added onto a project, and when a call
//! matches the name the VM first check if a hook is available for the function call. Each hook has
//! full access to the [VM] so they can re-implement most of the functionality.
//!
#![warn(rust_2018_idioms, rust_2021_compatibility)]
//#![warn(missing_docs)]
#![allow(clippy::module_inception)] // For now.

pub mod hooks;
pub mod memory;
pub mod project;
pub mod solver;
pub mod traits;
pub mod vm;

pub use solver::{Solutions, Solver, BV};

pub use crate::{
    project::Project,
    vm::{Result, ReturnValue, VMError, VM},
};

/// Assume the condition.
///
/// Adds a contraint that the passed condition must be true. If the condition can never be true,
/// this will lead to an `Unsat` error.
///
/// # Example
///
/// ```rust
/// # use x0001e::assume;
/// fn foo(var: i32) -> i32 {
///     // Will add a contraint to the solver for the passed condition.
///     assume(var >= 0);
///     if var < 0 {
///         unreachable!();
///     }
///     var
/// }
/// ```
#[inline(never)]
#[allow(unused_variables)]
pub fn assume(condition: bool) {
    // Implemented as hook `hooks::assume`.
}

/// Creates a new symbolic value for `value`. This removes all contraints.
///
/// This creates a new symbolic variable and assigns overwrites the passed `value`. This must be
/// performed since constraints added to the solver cannot be removed, and the previous value may
/// have constraints associated with it.
///
/// # Example
///
/// ```rust
/// # use x0001e::symbolic;
/// fn foo() {
///     // This will create a symbol with the contstraint that x is 0.
///     let mut x = 0;
///     // Removes all constraints from `x`, letting it be an unconstrained symbol
///     // that can be anything.
///     symbolic(&mut x);
///     if x != 0 {
///         // This path will be found
///     }
/// }
/// ```
#[inline(never)]
#[allow(unused_variables)]
pub fn symbolic<T>(value: &mut T) {
    // Implemented as hook `hooks::symbolic`.
}

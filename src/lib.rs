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
//!     let project = Project::from_path("tests/doc_tests/test.bc")?;
//!     let mut vm = VM::new("test::function_to_analyze", &project)?;
//!
//!     while let Some(path_result) = vm.run() {
//!         // Result of running one path is contained in the path_result.
//!         //
//!         // The VM state is as it was at the last executed instruction, and methods are available
//!         // on the VM to inspect the state and variables.
//!         //
//!         // Most interesting are likely `vm.parameters` for all input variables to the function,
//!         // and `vm.state.symbolic` for variables marked with `symbolic()`.
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
//!             .map(|input| vm.solver.get_solutions_for_bv(&input.value, 1))
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
//! The [Solver] keeps tracks of all the constraints added to it. It it also used to create new
//! symbols. The solver can be queried for solutions to different symbols.
//!
//! ## Hooks
//!
//! Hooks can be added for any function the IR calls. They are added onto a project, and when a call
//! matches the name the VM first check if a hook is available for the function call. Each hook has
//! full access to the [VM] so they can re-implement most of the functionality.
//!
// #![warn(rust_2018_idioms, rust_2021_compatibility)]
//#![warn(missing_docs)]

// mod environment;
mod executor;
mod memory;
mod smt;
mod util;

use std::path::Path;

pub use executor::llvm::{project::Project, LLVMExecutor};
pub use executor::vm::VM;
pub use executor::{Executor, ExecutorError};
pub use memory::MemoryError;

use self::executor::vm::ReturnValue;
use self::smt::DContext;
pub use executor::*;
pub use util::*;

pub fn run(path: impl AsRef<Path>, function: &str) -> Result<Vec<ReturnValue>, ExecutorError> {
    let context = Box::new(DContext::new());
    let context = Box::leak(context);

    let project = Box::new(Project::from_path(path).unwrap());
    let project = Box::leak(project);

    let mut vm = VM::new(project, context, function).unwrap();

    let mut results = Vec::new();
    while let Some(r) = vm.run() {
        println!("result: {r:?}");
        results.push(r.unwrap());
    }

    Ok(results)
}

// // pub use solver::{Solutions, Solver, BV};
// // pub use solver::*;
// pub use solver_z3::{Solutions, SolverError};
// type Solver = solver_z3::Solver<'static>;
// type BV = solver_z3::BV<'static>;

// pub use crate::{
//     project::Project,
//     vm::{Result, ReturnValue, VMError, VM},
// };

// pub mod ir {
//     ///! Re-exports of `llvm-ir` types.
//     pub use llvm_ir::types::NamedStructDef;
//     pub use llvm_ir::*;
// }

// pub fn create_ctx() -> &'static z3::Context {
//     let mut cfg = z3::Config::new();
//     cfg.set_model_generation(true);
//     let context = Box::new(z3::Context::new(&cfg));
//     let context = Box::leak(context);
//     context
// }

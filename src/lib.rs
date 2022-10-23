//! SYMEX aims to be a general purpose symbolic execution environment.
//!
//! # Background
//!
//!
//!
//!
//! # LLVM IR
//!
//! It aims to be a general purpose execution engine that can analyze mainly Rust code, but should
//! handle other languages that can generate LLVM bitcode.
//!
//! It currently supports LLVM 14, and uses the [boolector](https://crates.io/crates/boolector)
//! library as the SMT solver.
//!
//! # Example
//!
//! Assuming the code to analyze is
//!
//! ```ignore
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
//! use symex::{Project, ReturnValue, VM};
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
#![warn(rust_2018_idioms, rust_2021_compatibility)]
//#![warn(missing_docs)]

pub mod core;
pub mod llvm;
pub mod memory;
pub mod run;
pub mod smt;
pub mod util;

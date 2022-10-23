//! This module provides a symbolic execution engine that operates on LLVM IR.
//!
//! It aims to be a general purpose execution engine that can analyze mainly Rust code, but should
//! handle other languages that can generate LLVM bitcode. Current version for LLVM is 14.
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
//! # Note
//!
//! There is currently work being done to generalize parts of this to allow for other executors.
pub mod common;
pub mod environment;
pub mod error;
pub mod executor;
pub mod project;
pub mod state;
pub mod vm;

pub use error::LLVMExecutorError;

///
pub type Result<T> = std::result::Result<T, LLVMExecutorError>;

#[derive(Debug, Clone)]
pub struct Config {
    /// Maximum call stack depth. Default is `1000`.
    pub max_call_depth: usize,

    /// Maximum iteration count. Default is `1000`.
    pub max_iter_count: usize,

    /// Maximum amount of concretizations for function pointers. Default is `1`.
    pub max_fn_ptr_resolutions: usize,

    /// Maximum amount of concretizations for a memory address. This does not apply for e.g.
    /// ArrayMemory, but does apply for ObjectMemory. Default is `100`.
    pub max_memory_access_resolutions: usize,

    /// Maximum amount of concretizations for memmove, memcpy, memset and other intrisic functions.
    /// Default is `100`.
    pub max_intrinsic_concretizations: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Creates a new `Config` with default values.
    ///
    /// Current defaults are set to
    ///
    /// - `max_call_depth`: 1000
    /// - `max_iter_count`: 1000
    /// - `max_fn_ptr_resolutions`: 1
    /// - `max_memory_access_resolutions`: 100
    /// - `max_intrinsic_concretizations`: 100
    pub fn new() -> Self {
        Self {
            max_call_depth: 1000,
            max_iter_count: 1000,
            max_fn_ptr_resolutions: 1,
            max_memory_access_resolutions: 100,
            max_intrinsic_concretizations: 100,
        }
    }
}

#[derive(Debug)]
pub struct Stats {
    /// Number of instructions the executor has processed in total.
    pub instructions_processed: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

impl Stats {
    pub fn new() -> Self {
        Self {
            instructions_processed: 0,
        }
    }
}

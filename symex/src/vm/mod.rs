mod executor;
mod hooks;
mod intrinsic;
mod path_selection;
mod project;
mod state;
mod vm;

pub use executor::*;
pub use hooks::*;
pub use intrinsic::*;
pub use path_selection::*;
pub use project::*;
pub use state::*;
pub use vm::*;

use crate::{memory::MemoryError, smt::SolverError};

// TODO: Other errors
//
// - Cannot take size of opaque struct.
// - Cannot find named struct.
// - Cannot convert operand to symbol.
// - Global reference not found
// - Cannot find basic block.
// - Call depth exceeded.
// - Iteration count exceeded.

pub type Result<T> = std::result::Result<T, LLVMExecutorError>;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum LLVMExecutorError {
    #[error("Abort {0}")]
    Abort(i64),

    /// Function not found
    #[error("Function not found: {0}")]
    FunctionNotFound(String),

    /// Local register variable not found.
    #[error("Local not found: {0}")]
    LocalNotFound(String),

    #[error("Cannot take size of type")]
    NoSize,

    /// MalformedInstruction
    #[error("MalformedInstruction")]
    MalformedInstruction,

    /// UnsupportedInstruction
    #[error("UnsupportedInstruction {0}")]
    UnsupportedInstruction(String),

    /// UnreachableInstruction
    #[error("UnreachableInstruction")]
    UnreachableInstruction,

    #[error("UnexpectedZeroSize")]
    UnexpectedZeroSize,

    #[error("Path suppressed")]
    SuppressPath,

    #[error("No active stack frame")]
    NoStackFrame,

    #[error("Memory error")]
    MemoryError(#[from] MemoryError),

    #[error("Solver error")]
    SolverError(#[from] SolverError),
}

// /// Errors why a certain path failed.
// ///
// /// Indiviual errors from the specific VM/Executors should be converted to this more general error
// #[derive(Debug, thiserror::Error, PartialEq)]
// pub enum VMError {
//     #[error("{}", UNEXPECTED_PARAMETER_MESSAGE)]
//     UnexpectedParameter,

//     #[error("Abort {0}")]
//     Abort(i64),

//     #[error("SolverError")]
//     SolverError(#[from] SolverError),

//     #[error("MemoryError")]
//     MemoryError(#[from] MemoryError),

//     #[error("Other {0}")]
//     Other(String),
// }

// const UNEXPECTED_PARAMETER_MESSAGE: &str = r#"Parameters for functions are not supported.

// Function parameters are not supported by the system, wrap the function inside another that takes not parameters.

// use symbolic_lib::symbolic;
// fn function_under_test(a: [i32; 3]) {}
// fn wrapped() {
//     let mut a = [0; 3];
//     symbolic(&mut a);
//     function_under_test(a);
// }

// Note that returning larger values may also result in the compiler generating code that takes the return value as a parameter instead.
// "#;

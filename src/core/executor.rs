use super::{memory::MemoryError, smt::SolverError};

/// Errors why a certain path failed.
///
/// Indiviual errors from the specific VM/Executors should be converted to this more general error
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum VMError {
    #[error("{}", UNEXPECTED_PARAMETER_MESSAGE)]
    UnexpectedParameter,

    #[error("Abort {0}")]
    Abort(i64),

    #[error("SolverError")]
    SolverError(#[from] SolverError),

    #[error("MemoryError")]
    MemoryError(#[from] MemoryError),

    #[error("Other {0}")]
    Other(String),
}

const UNEXPECTED_PARAMETER_MESSAGE: &str = r#"Parameters for functions are not supported.

Function parameters are not supported by the system, wrap the function inside another that takes not parameters.

use symbolic_lib::symbolic;
fn function_under_test(a: [i32; 3]) {}
fn wrapped() {
    let mut a = [0; 3];
    symbolic(&mut a);
    function_under_test(a);
}

Note that returning larger values may also result in the compiler generating code that takes the return value as a parameter instead.
"#;

// WORK IN PROGRESS:
// Have to figure out a good set of traits for executor related functionality.
//
// Should support different executors such as for LLVM and ASM.
//
// To begin with it may be smarter to have a trait on the VM, and make it really coarse grained,
// keeping them completely separate and see which functions are similar (if any).
//pub trait Executor {}

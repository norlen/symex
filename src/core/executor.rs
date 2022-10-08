use super::{memory::MemoryError, smt::SolverError};

/// Errors why a certain path failed.
///
/// Indiviual errors from the specific VM/Executors should be converted to this more general error
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum VMError {
    #[error("Abort {0}")]
    Abort(i64),

    #[error("SolverError")]
    SolverError(#[from] SolverError),

    #[error("MemoryError")]
    MemoryError(#[from] MemoryError),

    #[error("Other {0}")]
    Other(String),
}

// WORK IN PROGRESS:
// Have to figure out a good set of traits for executor related functionality.
//
// Should support different executors such as for LLVM and ASM.
//
// To begin with it may be smarter to have a trait on the VM, and make it really coarse grained,
// keeping them completely separate and see which functions are similar (if any).
//pub trait Executor {}

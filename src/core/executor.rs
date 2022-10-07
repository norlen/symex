use super::{memory::MemoryError, smt::SolverError};

// TODO: Have to figure out a good set of traits for executor related functionality.

// Should support different executors such as for LLVM and ASM
pub trait Executor {}

// Indiviual errors from the specific executors should be converted to this more general error

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ExecutorError {
    #[error("Abort {0}")]
    Abort(i64),

    #[error("SolverError")]
    SolverError(#[from] SolverError),

    #[error("MemoryError")]
    MemoryError(#[from] MemoryError),

    #[error("Other {0}")]
    Other(String),
}

use crate::core::{executor::VMError, memory::MemoryError, smt::SolverError};

pub type Result<T> = std::result::Result<T, LLVMExecutorError>;

// TODO: Other errors
//
// - Cannot take size of opaque struct.
// - Cannot find named struct.
// - Cannot convert operand to symbol.
// - Global reference not found
// - Cannot find basic block.
// - Call depth exceeded.
// - Iteration count exceeded.

#[derive(Debug, thiserror::Error)]
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

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Memory error")]
    MemoryError(#[from] MemoryError),

    #[error("Solver error")]
    SolverError(#[from] SolverError),

    #[error("Path suppressed")]
    SuppressPath,
}

impl Into<VMError> for LLVMExecutorError {
    fn into(self) -> VMError {
        use LLVMExecutorError::*;
        match self {
            Abort(i) => VMError::Abort(i),
            MemoryError(e) => VMError::MemoryError(e),
            SolverError(e) => VMError::SolverError(e),
            _ => VMError::Other(format!("{self}")),
        }
    }
}

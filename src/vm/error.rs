use thiserror::Error;

pub type Result<T> = std::result::Result<T, VMError>;

#[derive(Debug, Error, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ExecutionError {
    #[error("Abort")]
    Abort,

    #[error("Throw")]
    Throw,
}

/// VMError
#[derive(Debug, Error, PartialEq, Eq)]
#[allow(dead_code)]
pub enum VMError {
    #[error("Execution err: {0}")]
    Execution(#[from] ExecutionError),

    /// Unsat
    #[error("Unsat")]
    Unsat,

    #[error("Out of bounds")]
    OutOfBounds,

    /// UnsupportedInstruction
    #[error("UnsupportedInstruction")]
    UnsupportedInstruction,

    /// MalformedInstruction
    #[error("MalformedInstruction")]
    MalformedInstruction,

    /// UnreachableInstruction
    #[error("UnreachableInstruction")]
    UnreachableInstruction,

    /// Function not found
    #[error("Function not found: {0}")]
    FunctionNotFound(String),
}

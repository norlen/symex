use thiserror::Error;

pub type Result<T> = std::result::Result<T, VMError>;

/// VMError
#[derive(Debug, Error, PartialEq, Eq)]
#[allow(dead_code)]
pub enum VMError {
    // -------------------------------------------------------------------------
    // Errors in the code being analyzed.
    // -------------------------------------------------------------------------
    #[error("Abort")]
    Abort,

    #[error("Throw")]
    Throw,

    #[error("Unsat")]
    Unsat,

    #[error("Out of bounds")]
    OutOfBounds,

    // -------------------------------------------------------------------------
    // Errors in IR
    // -------------------------------------------------------------------------
    /// Function not found
    #[error("Function not found: {0}")]
    FunctionNotFound(String),

    /// MalformedInstruction
    #[error("MalformedInstruction")]
    MalformedInstruction,

    // -------------------------------------------------------------------------
    // Errors when running the VM
    // -------------------------------------------------------------------------
    /// UnsupportedInstruction
    #[error("UnsupportedInstruction")]
    UnsupportedInstruction,

    /// UnreachableInstruction
    #[error("UnreachableInstruction")]
    UnreachableInstruction,
}

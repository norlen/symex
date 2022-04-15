use thiserror::Error;

use crate::memory::MemoryError;

pub type Result<T> = std::result::Result<T, VMError>;

pub type StatusCode = i64;

/// VMError
#[derive(Debug, Error)]
pub enum VMError {
    // -------------------------------------------------------------------------
    // Errors in the code being analyzed.
    // -------------------------------------------------------------------------
    #[error("Abort")]
    Abort(StatusCode),

    #[error("Throw")]
    Throw,

    #[error("Unsat")]
    Unsat,

    // #[error("Out of bounds")]
    // OutOfBounds,

    // -------------------------------------------------------------------------
    // Errors in IR
    // -------------------------------------------------------------------------
    /// Function not found
    #[error("Function not found: {0}")]
    FunctionNotFound(String),

    #[error("Local not found: {0}")]
    LocalNotFound(String),

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

    #[error("Expected type to be non-zero sized")]
    UnexpectedZeroSize,

    #[error(transparent)]
    MemoryError(#[from] MemoryError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl PartialEq for VMError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::FunctionNotFound(l0), Self::FunctionNotFound(r0)) => l0 == r0,
            (Self::Other(l0), Self::Other(r0)) => l0.to_string() == r0.to_string(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

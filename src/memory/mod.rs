use thiserror::Error;

use crate::solver::SolverError;

pub mod bump_allocator;
pub mod simple_memory;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MemoryError {
    /// Tried to allocate with a size of zero.
    #[error("Tried to allocate with a size of zero")]
    ZeroSizedAllocation,

    /// When wanting a size in bytes, if the bits don't cleanly map to a certain
    /// amount of bytes.
    #[error("Number of bits {0} is not a multiple of bytes")]
    BitsNotMultipleOfBytes(u64),

    /// The given size is not a power of two.
    #[error("Number of bits {0} is not a power of two")]
    NotPowerOfTwo(u64),

    /// When the address space becomes exhausted.
    #[error("Tried to allocate {0} bits which would overflow the address space")]
    AddressSpaceExhausted(u64),

    /// Possible to try and read/write a null pointer.
    #[error("Null pointer encountered")]
    NullPointer,

    #[error(transparent)]
    Solver(#[from] SolverError),
}

/// The number of bits per byte the memory system expects.
pub const BITS_IN_BYTE: u32 = 8;

/// Converts number of bits to bytes, returning an error if `bits` are not
/// a mulitple of `[BITS_IN_BYTE]`.
pub fn to_bytes(size: u64) -> Result<u64, MemoryError> {
    if size % BITS_IN_BYTE as u64 != 0 {
        Err(MemoryError::BitsNotMultipleOfBytes(size))
    } else {
        Ok(size / 8)
    }
}

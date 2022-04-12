use crate::vm::Result;
use anyhow::anyhow;

pub mod bump_allocator;
pub mod simple_memory;

/// The number of bits per byte the memory system expects.
pub const BITS_IN_BYTE: u32 = 8;

/// Converts number of bits to bytes, returning an error if `bits` are not
/// a mulitple of `[BITS_IN_BYTE]`.
pub fn to_bytes(size: u64) -> Result<u64> {
    if size % BITS_IN_BYTE as u64 != 0 {
        Err(anyhow!("invalid number of bits").into())
    } else {
        Ok(size / 8)
    }
}

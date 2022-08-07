//!
//!
//!
use tracing::debug;

use crate::memory::{MemoryError, BITS_IN_BYTE};

/// Simple bump allocator that starts allocating addresses at `LinearAllocator::ALLOC_START`
#[derive(Debug, Clone, Default)]
pub struct LinearAllocator {
    /// Pointer to next available address to allocate at.
    cursor: u64,
}

impl LinearAllocator {
    /// All allocations begin at this address.
    const ALLOC_START: u64 = 0x1000_0000;

    /// Create a new `LinearAllocator` that starts allocating at `ALLOC_START`.
    pub fn new() -> Self {
        Self {
            cursor: Self::ALLOC_START,
        }
    }

    /// Returns the first available address.
    ///
    /// `bits` must be the correct size, if it is too small subsequent
    /// allocations will be in the same address space.
    pub fn get_address(&mut self, bits: u64, align: u64) -> Result<(u64, u64), MemoryError> {
        if bits == 0 {
            return Err(MemoryError::ZeroSizedAllocation);
        }
        if !align.is_power_of_two() {
            return Err(MemoryError::NotPowerOfTwo(align));
        }

        // Find the next available aligned address.
        let start_addr_aligned = (self.cursor + align - 1) & !(align - 1);

        // Number of bytes requires for the amount of bits requested.
        let bytes = {
            let floored_bytes = bits / BITS_IN_BYTE as u64;
            if bits % BITS_IN_BYTE as u64 == 0 {
                floored_bytes
            } else {
                floored_bytes + 1
            }
        };

        let next_cursor = start_addr_aligned.wrapping_add(bytes);
        if next_cursor < self.cursor {
            return Err(MemoryError::AddressSpaceExhausted(bits));
        }
        self.cursor = next_cursor;

        debug!(
            "Allocated {} bits ({} bytes) at address: 0x{:x}",
            bits, bytes, start_addr_aligned
        );
        Ok((start_addr_aligned, bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_in_sequence() {
        let mut alloc = LinearAllocator::new();
        let addr = alloc.get_address(32, 1);
        let addr2 = alloc.get_address(32, 1);

        assert_eq!(addr, Ok((LinearAllocator::ALLOC_START, 4)));
        assert_eq!(addr2, Ok((LinearAllocator::ALLOC_START + 4, 4))); // 4*8 = 32 bits
    }

    #[test]
    fn allocate_align() {
        let mut alloc = LinearAllocator::new();
        let addr = alloc.get_address(16, 4);
        let addr2 = alloc.get_address(32, 4);

        assert_eq!(addr, Ok((LinearAllocator::ALLOC_START, 2)));
        assert_eq!(addr2, Ok((LinearAllocator::ALLOC_START + 4, 4))); // 4*8 = 32 bits
    }

    #[test]
    fn align_non_pow_two_fails() {
        let mut alloc = LinearAllocator::new();
        assert!(matches!(
            alloc.get_address(32, 3),
            Err(MemoryError::NotPowerOfTwo(_))
        ));
    }

    #[test]
    fn zero_sized_alloc_panics() {
        let mut alloc = LinearAllocator::new();
        assert_eq!(
            alloc.get_address(0, 4),
            Err(MemoryError::ZeroSizedAllocation)
        );
    }

    #[test]
    fn handles_overflow() {
        let mut alloc = LinearAllocator::new();
        for _ in 0..7 {
            assert!(alloc.get_address(u64::MAX, 4).is_ok());
        }
        assert_eq!(
            alloc.get_address(u64::MAX, 4),
            Err(MemoryError::AddressSpaceExhausted(u64::MAX))
        );
    }
}

use log::debug;
use std::collections::HashMap;

use super::{MemoryError, BITS_IN_BYTE};

/// Simple bump allocator that starts allocating addresses at [BumpAllocator::ALLOC_START]
#[derive(Debug, Clone, Default)]
pub struct BumpAllocator {
    /// Pointer to next avaiable address to allocate at.
    cursor: u64,

    /// Map allocation address to the allocated size in bits.
    sizes: HashMap<u64, u64>,
}

impl BumpAllocator {
    /// All allocations begin at this address.
    pub const ALLOC_START: u64 = 0x1000_0000;

    /// Create a new `BumpAllocator` that starts allocating at `ALLOC_START`.
    pub fn new() -> Self {
        Self {
            cursor: Self::ALLOC_START,
            sizes: HashMap::new(),
        }
    }

    pub fn get_allocated_size(&self, addr: u64) -> Option<u64> {
        self.sizes.get(&addr).copied()
    }

    /// Returns the first available address.
    ///
    /// `bits` must be the correct size, if it is too small subsequent
    /// allocations will be in the same address space.
    pub fn get_address(&mut self, bits: u64, align: u64) -> Result<u64, MemoryError> {
        if bits == 0 {
            return Err(MemoryError::ZeroSizedAllocation);
        }
        if !align.is_power_of_two() {
            return Err(MemoryError::NotPowerOfTwo(bits));
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
        self.sizes.insert(start_addr_aligned, bits);

        debug!(
            "Allocated {} bits ({} bytes) at address: 0x{:x}",
            bits, bytes, start_addr_aligned
        );
        Ok(start_addr_aligned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_in_sequence() {
        let mut alloc = BumpAllocator::new();
        let addr = alloc.get_address(32, 1);
        let addr2 = alloc.get_address(32, 1);

        assert_eq!(addr, Ok(BumpAllocator::ALLOC_START));
        assert_eq!(addr2, Ok(BumpAllocator::ALLOC_START + 4)); // 4*8 = 32 bits
    }

    #[test]
    fn allocate_align() {
        let mut alloc = BumpAllocator::new();
        let addr = alloc.get_address(16, 4);
        let addr2 = alloc.get_address(32, 4);

        assert_eq!(addr, Ok(BumpAllocator::ALLOC_START));
        assert_eq!(addr2, Ok(BumpAllocator::ALLOC_START + 4)); // 4*8 = 32 bits
    }

    #[test]
    fn align_non_pow_two_fails() {
        let mut alloc = BumpAllocator::new();
        assert!(matches!(
            alloc.get_address(32, 3),
            Err(MemoryError::NotPowerOfTwo(_))
        ));
    }

    #[test]
    fn zero_sized_alloc_panics() {
        let mut alloc = BumpAllocator::new();
        assert_eq!(
            alloc.get_address(0, 4),
            Err(MemoryError::ZeroSizedAllocation)
        );
    }

    #[test]
    fn handles_overflow() {
        let mut alloc = BumpAllocator::new();
        for _ in 0..7 {
            assert!(alloc.get_address(u64::MAX, 4).is_ok());
        }
        assert_eq!(
            alloc.get_address(u64::MAX, 4),
            Err(MemoryError::AddressSpaceExhausted(u64::MAX))
        );
    }
}

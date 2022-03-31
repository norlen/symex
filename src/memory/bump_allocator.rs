use super::BITS_IN_BYTE;
use log::debug;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct BumpAllocator {
    /// Pointer to next avaiable address to allocate at.
    cursor: usize,

    /// Map allocation address to the allocated size in bits.
    sizes: HashMap<usize, usize>,
}

impl BumpAllocator {
    /// All allocations begin at this address.
    pub const ALLOC_START: usize = 0x1000_0000;

    /// Create a new `BumpAllocator` that starts allocating at `ALLOC_START`.
    pub fn new() -> Self {
        Self {
            cursor: Self::ALLOC_START,
            sizes: HashMap::new(),
        }
    }

    /// Returns the first available address.
    ///
    /// `bits` must be the correct size, if it is too small subsequent
    /// allocations will be in the same address space.
    pub fn get_address(&mut self, bits: usize, align: usize) -> usize {
        if bits == 0 {
            panic!("Zero sized allocation was requested");
        }
        if !align.is_power_of_two() {
            panic!("alignment must be a power of two");
        }

        // Find the next available aligned address.
        let start_addr_aligned = (self.cursor + align - 1) & !(align - 1);

        // Number of bytes requires for the amount of bits requested.
        let bytes = {
            let floored_bytes = bits / BITS_IN_BYTE as usize;
            if bits % BITS_IN_BYTE as usize == 0 {
                floored_bytes
            } else {
                floored_bytes + 1
            }
        };

        let next_cursor = start_addr_aligned + bytes;
        if next_cursor < self.cursor {
            panic!("overflow");
        }
        self.cursor = next_cursor;
        self.sizes.insert(start_addr_aligned, bits);

        debug!(
            "Allocated {} bits ({} bytes) at address: 0x{:x}",
            bits, bytes, start_addr_aligned
        );
        start_addr_aligned
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_in_sequence() {
        let mut alloc = BumpAllocator::new();
        let addr = alloc.get_address(32usize, 1usize);
        let addr2 = alloc.get_address(32usize, 1usize);

        assert_eq!(addr, BumpAllocator::ALLOC_START);
        assert_eq!(addr2, BumpAllocator::ALLOC_START + 4); // 4*8 = 32 bits
    }

    #[test]
    fn allocate_align() {
        let mut alloc = BumpAllocator::new();
        let addr = alloc.get_address(16usize, 4usize);
        let addr2 = alloc.get_address(32usize, 4usize);

        assert_eq!(addr, BumpAllocator::ALLOC_START);
        assert_eq!(addr2, BumpAllocator::ALLOC_START + 4); // 4*8 = 32 bits
    }

    #[test]
    #[should_panic]
    fn align_non_pow_two_fails() {
        let mut alloc = BumpAllocator::new();
        alloc.get_address(32, 3);
    }

    #[test]
    #[should_panic]
    fn zero_sized_alloc_panics() {
        let mut alloc = BumpAllocator::new();
        alloc.get_address(0, 4);
    }

    #[test]
    #[should_panic]
    fn handles_overflow() {
        let mut alloc = BumpAllocator::new();
        for _ in 0..10 {
            alloc.get_address(usize::MAX, 4);
        }
    }
}

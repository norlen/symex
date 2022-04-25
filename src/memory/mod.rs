use log::{debug, trace};
use thiserror::Error;

use crate::{
    solver::{Array, SolverError},
    Solver, BV,
};

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

    #[error("Out of bounds")]
    OutOfBounds,
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

#[derive(Debug, Clone)]
pub struct NewMemory {
    solver: Solver,

    allocator: BumpAllocator2,

    store: MemoryStore,

    allocations: MemoryStore,

    ptr_size: u32,

    null_detection: bool,

    nullptr: BV,

    next_allocation_id: usize,
}

impl NewMemory {
    pub fn new(solver: Solver, ptr_size: u32) -> Self {
        let nullptr = solver.bv_zero(ptr_size);
        let store = MemoryStore::new_uninitialized("memory", &solver, ptr_size);
        let allocations = MemoryStore::new_uninitialized("allocations", &solver, ptr_size);

        Self {
            solver,
            allocator: BumpAllocator2::new(),
            store,
            allocations,
            ptr_size,
            null_detection: false,
            nullptr,
            next_allocation_id: 0,
        }
    }

    pub fn allocate(&mut self, bits: u64, align: u64) -> Result<u64, MemoryError> {
        let (addr, bytes) = self.allocator.get_address(bits, align)?;

        println!(
            "allocate addr: {addr:x}, bytes: {bytes}, allocation_id: {}",
            self.next_allocation_id
        );
        let alloc_id = self.solver.bv_from_u64(self.next_allocation_id as u64, 8);
        for i in 0..bytes {
            println!("ALLOC ID {alloc_id:?}: ADDR {:x}", addr + i);
            let addr = self.solver.bv_from_u64(addr + i, self.ptr_size);
            self.allocations.write_u8(&addr, &alloc_id);
        }
        self.next_allocation_id += 1;

        Ok(addr)
    }

    pub fn check_same_alloc(&self, base_addr: &BV, addr: &BV) -> Result<(), MemoryError> {
        let base_alloc_id = self.allocations.read_u8(base_addr);
        let end_id = self.allocations.read_u8(&addr);

        if !self.solver.must_be_equal(&base_alloc_id, &end_id)? {
            return Err(MemoryError::OutOfBounds);
        }
        Ok(())
    }

    pub fn read(&self, addr: &BV, bits: u32) -> Result<BV, MemoryError> {
        trace!("{} read addr={:?}, bits={}", self.store.name, addr, bits);
        assert_eq!(addr.len(), self.ptr_size, "passed wrong sized address");

        // Check that the address cannot be null.
        if self.null_detection && self.solver.can_equal(addr, &self.nullptr)? {
            return Err(MemoryError::NullPointer);
        }

        // If we try to read more than a single byte, check that the read does not cross into
        // another allocation.
        if bits > BITS_IN_BYTE {
            let last_byte = (bits / 8) as u64 - 1;
            let last_byte = self.solver.bv_from_u64(last_byte, self.ptr_size);
            let end_addr = addr.add(&last_byte);

            let start_alloc_id = self.allocations.read_u8(addr);
            let end_alloc_id = self.allocations.read_u8(&end_addr);

            // These must be equal, otherwise the read is across an allocation boundary.
            if !self.solver.must_be_equal(&start_alloc_id, &end_alloc_id)? {
                return Err(MemoryError::OutOfBounds);
            }
        }

        self.store.read(addr, bits, &self.solver, self.ptr_size)
    }

    pub fn write(&mut self, addr: &BV, value: BV) -> Result<(), MemoryError> {
        trace!(
            "{} write addr={:?}, value={:?}",
            self.store.name,
            addr,
            value
        );
        assert_eq!(addr.len(), self.ptr_size, "passed wrong sized address");

        // Check that the address cannot be null.
        if self.null_detection && self.solver.can_equal(addr, &self.nullptr)? {
            return Err(MemoryError::NullPointer);
        }

        self.store.write(addr, value, &self.solver, self.ptr_size)
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStore {
    name: &'static str,

    mem: Array,
}

impl MemoryStore {
    pub fn new_uninitialized(name: &'static str, solver: &Solver, ptr_size: u32) -> Self {
        let mem = solver.array(ptr_size, BITS_IN_BYTE, Some(name));

        Self { name, mem }
    }

    pub fn read_u8(&self, addr: &BV) -> BV {
        self.mem.read(addr)
    }

    pub fn write_u8(&mut self, addr: &BV, val: &BV) {
        self.mem = self.mem.write(addr, val);
    }

    pub fn read(
        &self,
        addr: &BV,
        bits: u32,
        solver: &Solver,
        ptr_size: u32,
    ) -> Result<BV, MemoryError> {
        let value = if bits < BITS_IN_BYTE {
            self.read_u8(addr).slice(bits - 1, 0)
        } else {
            // Ensure we only read full bytes now.
            assert_eq!(bits % BITS_IN_BYTE, 0, "Must read bytes, if bits >= 8");
            let num_bytes = bits / BITS_IN_BYTE;

            let mut bytes = Vec::new();
            for byte in 0..num_bytes {
                let offset = solver.bv_from_u64(byte as u64, ptr_size);
                let read_addr = addr.add(&offset);
                let value = self.read_u8(&read_addr);
                bytes.push(value);
            }

            bytes.into_iter().reduce(|acc, v| v.concat(&acc)).unwrap()
        };

        Ok(value)
    }

    pub fn write(
        &mut self,
        addr: &BV,
        value: BV,
        solver: &Solver,
        ptr_size: u32,
    ) -> Result<(), MemoryError> {
        // Check if we should zero extend the value (if it less than 8-bits).
        let value = if value.len() < BITS_IN_BYTE {
            value.zero_ext(BITS_IN_BYTE)
        } else {
            value
        };

        // Ensure the value we write is a multiple of `BITS_IN_BYTE`.
        assert_eq!(value.len() % BITS_IN_BYTE, 0);

        let num_bytes = value.len() / BITS_IN_BYTE;
        for n in 0..num_bytes {
            let low_bit = n * BITS_IN_BYTE;
            let high_bit = (n + 1) * BITS_IN_BYTE - 1;
            let byte = value.slice(low_bit, high_bit);

            let offset = solver.bv_from_u64(n as u64, ptr_size);
            let addr = addr.add(&offset);
            self.write_u8(&addr, &byte);
        }

        Ok(())
    }
}

/// Simple bump allocator that starts allocating addresses at [BumpAllocator::ALLOC_START]
#[derive(Debug, Clone, Default)]
pub struct BumpAllocator2 {
    /// Pointer to next avaiable address to allocate at.
    cursor: u64,
}

impl BumpAllocator2 {
    /// All allocations begin at this address.
    pub const ALLOC_START: u64 = 0x1000_0000;

    /// Create a new `BumpAllocator` that starts allocating at `ALLOC_START`.
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

        debug!(
            "Allocated {} bits ({} bytes) at address: 0x{:x}",
            bits, bytes, start_addr_aligned
        );
        Ok((start_addr_aligned, bytes))
    }
}

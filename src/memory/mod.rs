//! Memory provides the memory system used by the execution engine.
//!
//! The main struct [Memory] handles allocations for all purposes. This includes the variables
//! allocated on the stack. And should also be used for heap allocations.
//!
//! Memory should first be allocated, this allocates a certain amount of bits and returns an address
//! to an uninitialized piece of memory. Read/Write operations to those addresses can then be done.
//!
//! On allocations, the allocation is given a unique ID. This ID is used to ensure reads to not
//! cross allocation boundaries, and if they do a `MemoryError::OutOfBounds` is returned.
//!
//! The system also provides null pointer checking, this is enabled with `null_detection` in
//! [Memory]. This checks that the address cannot be null when both reading and writing.
//!
//! It does not currently check that reads are not performed from uninitialized memory.
use log::{debug, trace};
use thiserror::Error;

use crate::solver::{Array, Solver, SolverError, BV};

/// Feature flag that enables checking when reading that the first byte read matches the last byte
/// read's allocation id.
pub const CHECK_OUT_OF_BOUNDS: bool = false;

/// Error representing an issue when performing memory operations.
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

    /// Each allocation has a respective size, this is returned when a read starts inside one
    /// allocation and ends outside of it.
    #[error("Out of bounds")]
    OutOfBounds,

    /// Errors passed on from the solver.
    #[error(transparent)]
    Solver(#[from] SolverError),
}

/// The number of bits per byte the memory system expects.
pub const BITS_IN_BYTE: u32 = 8;

/// Converts number of bits to bytes, returning an error if `bits` are not a multiple of
/// `[BITS_IN_BYTE]`.
pub fn to_bytes(size: u64) -> Result<u64, MemoryError> {
    if size % BITS_IN_BYTE as u64 != 0 {
        Err(MemoryError::BitsNotMultipleOfBytes(size))
    } else {
        Ok(size / 8)
    }
}

/// Allocations and backing memory store.
///
/// Memory keeps track of all allocations and provides the backing memory store. All allocations
/// are tagged with an id. This allows checking for out of bounds reads.
///
/// With `null_detection` enabled this allows to to check for the possibility of the address being
/// null.
#[derive(Debug, Clone)]
pub struct Memory {
    /// Reference to the solver so new symbols can be created.
    solver: Solver,

    /// Allocator is used to generate new addresses.
    allocator: BumpAllocator,

    /// The actual memory. Stores all values written to memory.
    store: MemoryStore,

    /// Size of a pointer.
    ptr_size: u32,

    /// Enable to check if addresses passed to read or write can be null.
    null_detection: bool,

    /// Symbol that is a null pointer, used for comparisons if `null_detection` is enabled.
    nullptr: BV,

    /// Keep track of allocation IDs.
    ///
    /// Each allocation is given a certain ID. This stores all these IDs, so we can check for out
    /// of bounds reads.
    allocations: Array,

    /// The next allocation ID to store in `allocations`.
    next_allocation_id: usize,
}

impl Memory {
    /// Creates a new memory containing only uninitialized memory.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use x0001e::memory::Memory;
    /// # use x0001e::solver::Solver;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let solver = Solver::new();
    ///     let ptr_size = 64;
    ///     let mut memory = Memory::new(solver, ptr_size);
    /// #   Ok(())
    /// # }
    pub fn new(solver: Solver, ptr_size: u32) -> Self {
        let nullptr = solver.bv_zero(ptr_size);
        let store = MemoryStore::new_uninitialized("memory", &solver, ptr_size);
        let allocations = solver.array(ptr_size, 8, Some("allocations"));

        Self {
            solver,
            allocator: BumpAllocator::new(),
            store,
            allocations,
            ptr_size,
            null_detection: false,
            nullptr,
            next_allocation_id: 0,
        }
    }

    /// Allocate a new address in memory with `bits` aligned to `align`.
    ///
    /// This only creates an address and makes sure no other address is generated from
    /// `[address, address+bits)`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use x0001e::memory::Memory;
    /// # use x0001e::solver::Solver;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #   let solver = Solver::new();
    /// #   let ptr_size = 64;
    ///     let mut memory = Memory::new(solver, ptr_size);
    ///
    ///     let address = memory.allocate(32, 4)?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn allocate(&mut self, bits: u64, align: u64) -> Result<u64, MemoryError> {
        let (addr, bytes) = self.allocator.get_address(bits, align)?;

        debug!(
            "allocate addr: {addr:x}, bytes: {bytes}, allocation_id: {}",
            self.next_allocation_id
        );

        if CHECK_OUT_OF_BOUNDS {
            let alloc_id = self.solver.bv_from_u64(self.next_allocation_id as u64, 8);
            for i in 0..bytes {
                let addr = self.solver.bv_from_u64(addr + i, self.ptr_size);
                self.allocations = self.allocations.write(&addr, &alloc_id);
            }
            self.next_allocation_id += 1;
        }

        Ok(addr)
    }

    /// Read `bits` starting from `addr`.
    ///
    /// # Errors
    ///
    /// If the address can be null and `null_detection` is enabled, if the address can be null a
    /// `MemoryError::NullPointer` is returned.
    ///
    /// If the address leads to one allocation id, and `addr + bits` leads to another
    /// `MemoryError::OutOfBounds` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use x0001e::memory::Memory;
    /// # use x0001e::solver::Solver;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #   let solver = Solver::new();
    /// #   let ptr_size = 64;
    /// #
    ///     let mut memory = Memory::new(solver.clone(), ptr_size);
    ///     let address = memory.allocate(32, 4)?;
    ///     let address = solver.bv_from_u64(address, ptr_size);
    ///
    ///     let value = memory.read(&address, 32)?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn read(&self, addr: &BV, bits: u32) -> Result<BV, MemoryError> {
        trace!("{} read addr={:?}, bits={}", self.store.name, addr, bits);
        assert_eq!(addr.len(), self.ptr_size, "passed wrong sized address");
        self.check_null_ptr(addr)?;

        // If we try to read more than a single byte, check that the read does not cross into
        // another allocation.
        if CHECK_OUT_OF_BOUNDS && bits > BITS_IN_BYTE {
            self.check_out_of_bounds_by_size(addr, bits)?;
        }

        let val = self.store.read(addr, bits, &self.solver, self.ptr_size)?;
        trace!("Read value: {val:?}");
        Ok(val)
    }

    /// Writes `value` to the address `addr`.
    ///
    /// # Errors
    ///
    /// If the address can be null and `null_detection` is enabled, if the address can be null a
    /// `MemoryError::NullPointer` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use x0001e::memory::Memory;
    /// # use x0001e::solver::Solver;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #   let solver = Solver::new();
    /// #   let ptr_size = 64;
    /// #
    ///     let mut memory = Memory::new(solver.clone(), ptr_size);
    ///     let address = memory.allocate(32, 4)?;
    ///     let address = solver.bv_from_u64(address, ptr_size);
    ///     let value = solver.bv_from_u64(42, 32);
    ///
    ///     memory.write(&address, value)?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn write(&mut self, addr: &BV, value: BV) -> Result<(), MemoryError> {
        trace!(
            "{} write addr={:?}, value={:?}",
            self.store.name,
            addr,
            value
        );
        assert_eq!(addr.len(), self.ptr_size, "passed wrong sized address");
        self.check_null_ptr(addr)?;
        self.store.write(addr, value, &self.solver, self.ptr_size)
    }

    /// Check that the address cannot be null. Returns `MemoryError::NullPointer` if it is possible
    /// for the address to be zero.
    fn check_null_ptr(&self, addr: &BV) -> Result<(), MemoryError> {
        // Check that the address cannot be null.
        if self.null_detection && self.solver.can_equal(addr, &self.nullptr)? {
            Err(MemoryError::NullPointer)
        } else {
            Ok(())
        }
    }

    /// Check that `start_addr` and `start_addr + bits` are part of the same allocation. Returns
    /// `MemoryError::OutOfBounds` if the the two addresses are not part of the same allocation.
    fn check_out_of_bounds_by_size(&self, start_addr: &BV, bits: u32) -> Result<(), MemoryError> {
        let last_byte = (bits / 8) as u64 - 1;
        let last_byte = self.solver.bv_from_u64(last_byte, self.ptr_size);

        let end_addr = start_addr.add(&last_byte);

        self.outside_address_space(start_addr, &end_addr)?;
        self.check_out_of_bounds(start_addr, &end_addr)
    }

    /// Check that `start_addr` and `end_addr` are part of the same allocation. Returns
    /// `MemoryError::OutOfBounds` if the two addresses are not part of the same allocation.
    fn check_out_of_bounds(&self, start_addr: &BV, end_addr: &BV) -> Result<(), MemoryError> {
        let start_id = self.allocations.read(start_addr);
        let end_id = self.allocations.read(end_addr);

        // These must be equal, otherwise the read is across an allocation boundary.
        if !self.solver.must_be_equal(&start_id, &end_id)? {
            Err(MemoryError::OutOfBounds)
        } else {
            Ok(())
        }
    }

    /// Check if the address is outside of any allocated address.
    fn outside_address_space(&self, start_addr: &BV, end_addr: &BV) -> Result<(), MemoryError> {
        let start = self
            .solver
            .bv_from_u64(BumpAllocator::ALLOC_START, self.ptr_size);

        let end = self
            .solver
            .bv_from_u64(self.allocator.cursor, self.ptr_size);

        let c0 = start_addr.ugte(&start);
        let c1 = end_addr.ult(&end);

        if self.solver.is_sat_with_constraints(&[&c0, &c1])? {
            Ok(())
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
}

/// Simple bump allocator that starts allocating addresses at `BumpAllocator::ALLOC_START`
#[derive(Debug, Clone, Default)]
struct BumpAllocator {
    /// Pointer to next available address to allocate at.
    cursor: u64,
}

impl BumpAllocator {
    /// All allocations begin at this address.
    const ALLOC_START: u64 = 0x1000_0000;

    /// Create a new `BumpAllocator` that starts allocating at `ALLOC_START`.
    fn new() -> Self {
        Self {
            cursor: Self::ALLOC_START,
        }
    }

    /// Returns the first available address.
    ///
    /// `bits` must be the correct size, if it is too small subsequent
    /// allocations will be in the same address space.
    fn get_address(&mut self, bits: u64, align: u64) -> Result<(u64, u64), MemoryError> {
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

/// Simple backing memory store.
#[derive(Debug, Clone)]
struct MemoryStore {
    /// Name of the store, also used as the name of the underlying memory array.
    name: &'static str,

    /// Memory as a bitvector array.
    memory: Array,
}

impl MemoryStore {
    /// Create new uninitialized memory.
    ///
    /// The addresses are of `ptr_size` and they point to elements with a size of `BITS_IN_BYTE`s.
    fn new_uninitialized(name: &'static str, solver: &Solver, ptr_size: u32) -> Self {
        let memory = solver.array(ptr_size, BITS_IN_BYTE, Some(name));
        Self { name, memory }
    }

    /// Reads an u8 from the given address.
    fn read_u8(&self, addr: &BV) -> BV {
        self.memory.read(addr)
    }

    /// Writes an u8 value to the given address.
    fn write_u8(&mut self, addr: &BV, val: &BV) {
        self.memory = self.memory.write(addr, val);
    }

    /// Reads `bits` from `addr.
    ///
    /// If the number of bits are less than `BITS_IN_BYTE` then individual bits can be read, but
    /// if the number of bits exceed `BITS_IN_BYTE` then full bytes must be read.
    fn read(
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

    /// Writes `value` to the given `addr`.
    ///
    /// If the size of the value is less than `BITS_IN_BYTE` it will be zero-extended into a full
    /// byte. If the size exceeds `BITS_IN_BYTE` then the size **must** be a multiple of
    /// `BITS_IN_BYTE`.
    fn write(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocates_in_sequence() {
        let mut alloc = BumpAllocator::new();
        let addr = alloc.get_address(32, 1);
        let addr2 = alloc.get_address(32, 1);

        assert_eq!(addr, Ok((BumpAllocator::ALLOC_START, 4)));
        assert_eq!(addr2, Ok((BumpAllocator::ALLOC_START + 4, 4))); // 4*8 = 32 bits
    }

    #[test]
    fn allocate_align() {
        let mut alloc = BumpAllocator::new();
        let addr = alloc.get_address(16, 4);
        let addr2 = alloc.get_address(32, 4);

        assert_eq!(addr, Ok((BumpAllocator::ALLOC_START, 2)));
        assert_eq!(addr2, Ok((BumpAllocator::ALLOC_START + 4, 4))); // 4*8 = 32 bits
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

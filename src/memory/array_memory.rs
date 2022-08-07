//! Theories-of-array memory.
//!
//! This memory model uses theories-of-arrays and supports arbitrary read and writes with symbolic
//! values. It uses a linear address space which is byte addressable. A single write will split
//! the symbolic value into byte sized chunks, and write each individually into memory. A read will
//! concatenate multiple bytes into a single large symbol.
//!
//! The concatenation on reads will generate more complex expressions compared to other memory
//! models, and in general this memory model is slower compared to e.g. object memory. However,
//! it may provide better performance in certain situations.
use tracing::trace;

use super::{linear_allocator::LinearAllocator, Memory, BITS_IN_BYTE};
use crate::{Array, DArray, DContext, DExpr, Expression, MemoryError, SolverContext};

/// Allocations and backing memory store.
///
/// Memory keeps track of all allocations and provides the backing memory store. All allocations
/// are tagged with an id. This allows checking for out of bounds reads.
///
/// With `null_detection` enabled this allows to to check for the possibility of the address being
/// null.
#[derive(Debug, Clone)]
pub struct ArrayMemory {
    /// Reference to the context so new symbols can be created.
    ctx: DContext,

    /// Allocator is used to generate new addresses.
    allocator: LinearAllocator,

    /// Size of a pointer.
    ptr_size: u32,

    /// The actual memory. Stores all values written to memory.
    memory: DArray,
}

impl Memory for ArrayMemory {
    #[tracing::instrument(skip(self))]
    fn allocate(&mut self, bits: u64, align: u64) -> Result<u64, MemoryError> {
        let (addr, _) = self.allocator.get_address(bits, align)?;
        Ok(addr)
    }

    #[tracing::instrument(skip(self))]
    fn resolve_addresses(
        &self,
        addr: &DExpr,
        _upper_bound: usize,
    ) -> Result<Vec<DExpr>, MemoryError> {
        Ok(vec![addr.clone()])
    }

    #[tracing::instrument(skip(self))]
    fn read(&self, addr: &DExpr, bits: u32) -> Result<DExpr, MemoryError> {
        assert_eq!(addr.len(), self.ptr_size, "passed wrong sized address");

        let value = self.internal_read(addr, bits, self.ptr_size)?;
        trace!("Read value: {value:?}");
        Ok(value)
    }

    #[tracing::instrument(skip(self))]
    fn write(&mut self, addr: &DExpr, value: DExpr) -> Result<(), MemoryError> {
        assert_eq!(addr.len(), self.ptr_size, "passed wrong sized address");
        self.internal_write(addr, value, self.ptr_size)
    }
}

impl ArrayMemory {
    /// Creates a new memory containing only uninitialized memory.
    pub fn new(ctx: DContext, ptr_size: u32) -> Self {
        let memory = DArray::new(&ctx, ptr_size as usize, BITS_IN_BYTE as usize, "memory");

        Self {
            ctx,
            allocator: LinearAllocator::new(),
            ptr_size,
            memory,
        }
    }

    /// Reads an u8 from the given address.
    fn read_u8(&self, addr: &DExpr) -> DExpr {
        self.memory.read(addr)
    }

    /// Writes an u8 value to the given address.
    fn write_u8(&mut self, addr: &DExpr, val: DExpr) {
        self.memory.write(addr, val);
    }

    /// Reads `bits` from `addr.
    ///
    /// If the number of bits are less than `BITS_IN_BYTE` then individual bits can be read, but
    /// if the number of bits exceed `BITS_IN_BYTE` then full bytes must be read.
    fn internal_read(&self, addr: &DExpr, bits: u32, ptr_size: u32) -> Result<DExpr, MemoryError> {
        let value = if bits < BITS_IN_BYTE {
            self.read_u8(addr).slice(bits - 1, 0)
        } else {
            // Ensure we only read full bytes now.
            assert_eq!(bits % BITS_IN_BYTE, 0, "Must read bytes, if bits >= 8");
            let num_bytes = bits / BITS_IN_BYTE;

            let mut bytes = Vec::new();
            for byte in 0..num_bytes {
                let offset = self.ctx.from_u64(byte as u64, ptr_size);
                let read_addr = addr.add(&offset);
                let value = self.read_u8(&read_addr);
                bytes.push(value);
            }

            bytes.into_iter().reduce(|acc, v| v.concat(&acc)).unwrap()
        };

        Ok(value)
    }

    fn internal_write(
        &mut self,
        addr: &DExpr,
        value: DExpr,
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

            let offset = self.ctx.from_u64(n as u64, ptr_size);
            let addr = addr.add(&offset);
            self.write_u8(&addr, byte);
        }

        Ok(())
    }
}

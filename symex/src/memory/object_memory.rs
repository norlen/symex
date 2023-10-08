//! Object memory
//!
use std::collections::BTreeMap;
use tracing::{trace, warn};

use super::MemoryError;
use crate::{
    memory::linear_allocator::LinearAllocator,
    smt::{DContext, DExpr, DSolver, Solutions},
};

#[derive(Debug, Clone)]
pub struct MemoryObject {
    address: u64,

    size: u64,

    bv: DExpr,
}

impl MemoryObject {
    pub fn bit_size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug, Clone)]
pub struct ObjectMemory {
    ctx: &'static DContext,

    /// Allocator is used to generate new addresses.
    allocator: LinearAllocator,

    objects: BTreeMap<u64, MemoryObject>,

    solver: DSolver,

    ptr_size: u32,

    alloc_id: usize,
}

impl ObjectMemory {
    pub fn new(ctx: &'static DContext, ptr_size: u32, solver: DSolver) -> Self {
        Self {
            ctx,
            allocator: LinearAllocator::new(),
            objects: BTreeMap::new(),
            ptr_size,
            alloc_id: 0,
            solver,
        }
    }

    pub fn get_object(&self, address: u64) -> Option<&MemoryObject> {
        self.objects.get(&address)
    }

    /// Allocate `bits` of memory returning the newly allocated address.
    pub fn allocate(&mut self, bits: u64, align: u64) -> Result<u64, MemoryError> {
        let (addr, _bytes) = self.allocator.get_address(bits, align)?;

        let name = format!("alloc{}-{}", self.alloc_id, rand::random::<u32>());
        // println!("Allocate {name} at addr: 0x{addr:x}, size_bits: {bits} bytes");
        self.alloc_id += 1;

        let obj = MemoryObject {
            //name: name.clone(),
            address: addr,
            size: bits,
            bv: self.ctx.unconstrained(bits as u32, &name),
        };
        self.objects.insert(addr, obj);

        Ok(addr)
    }

    /// Read `bits` from `address`.
    #[tracing::instrument(skip(self))]
    pub fn read(&self, addr: &DExpr, bits: u32) -> Result<DExpr, MemoryError> {
        trace!("read addr={addr:?}, bits={bits}");
        assert_eq!(addr.len(), self.ptr_size, "passed wrong sized address");

        let (addr, value) = self.resolve_address(addr)?;
        let offset = (addr - value.address) as u32 * 8;
        let val = value.bv.slice(offset, offset + bits - 1);

        trace!("Return {val:?}, value: {value:x?}");
        Ok(val)
    }

    /// Write a value to `address`.
    #[tracing::instrument(skip(self))]
    pub fn write(&mut self, addr: &DExpr, value: DExpr) -> Result<(), MemoryError> {
        trace!("write addr={addr:?}, len={}, value={value:?}", value.len());
        // println!("write addr={addr:?}, len={}, value={value:?}", value.len());
        assert_eq!(addr.len(), self.ptr_size, "passed wrong sized address");

        let (addr, val) = self.resolve_address_mut(addr)?;
        let offset = (addr - val.address) * 8;
        // println!("mem obj: addr: {addr:x}, val: {val:#?}, offset: {offset}",);

        if value.len() == val.size as u32 {
            val.bv = value;
        } else {
            val.bv = val.bv.replace_part(offset as u32, value);
        }

        Ok(())
    }

    /// For a symbolic address, get addresses to read or write from.
    ///
    /// Certain memory models may not support fully symbolic pointers. This function allows the
    /// memory model to resolve the passed address to any number of locations. The caller is then
    /// expected to handle the multiple returned addresses by e.g. forking the path of execution.
    pub fn resolve_addresses(
        &self,
        address: &DExpr,
        upper_bound: usize,
    ) -> Result<Vec<DExpr>, MemoryError> {
        // Fast path if address is a constant.
        if let Some(_) = address.get_constant() {
            return Ok(vec![address.clone()]);
        }

        println!("Resolve addresses: {address:?}");

        // Otherwise, get solutions for addresses.
        let addresses = self.solver.get_values(address, upper_bound)?;
        let addresses = match addresses {
            Solutions::Exactly(s) => s,
            Solutions::AtLeast(s) => {
                warn!(
                    "More than {} possible addresses, suppressing other paths",
                    upper_bound
                );
                s
            }
        };
        println!("Addresses: {addresses:?}");

        Ok(addresses)
    }

    fn resolve_address(&self, address: &DExpr) -> Result<(u64, &MemoryObject), MemoryError> {
        let address = address.get_constant().unwrap();

        // Get the memory object with the address that is the closest below the passed address.
        for obj in self.objects.range(0..=address).rev().take(1) {
            // TODO: Perform bounds check.
            return Ok((address, obj.1));
        }

        panic!("Memory object not found");
    }

    fn resolve_address_mut(
        &mut self,
        address: &DExpr,
    ) -> Result<(u64, &mut MemoryObject), MemoryError> {
        let address = address.get_constant().unwrap();

        // Get the memory object with the address that is the closest below the passed address.
        for obj in self.objects.range_mut(0..=address).rev().take(1) {
            // TODO: Perform bounds check.
            return Ok((address, obj.1));
        }

        panic!("Memory object not found");
    }
}

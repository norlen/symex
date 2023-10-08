//! Object memory
//!
use std::collections::BTreeMap;
use tracing::{trace, warn};

use crate::{
    core::{
        memory::{Memory, MemoryError},
        smt::{Expression, Solutions, Solver, SolverContext},
    },
    memory::linear_allocator::LinearAllocator,
    smt::{DContext, DExpr, DSolver},
};

#[derive(Debug, Clone)]
pub struct MemoryObject {
    address: u64,

    size: u64,

    bv: DExpr,
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

impl Memory for ObjectMemory {
    fn allocate(&mut self, bits: u64, align: u64) -> Result<u64, MemoryError> {
        let (addr, _bytes) = self.allocator.get_address(bits, align)?;

        let name = format!("alloc{}", self.alloc_id);
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

    #[tracing::instrument(skip(self))]
    fn read(&self, addr: &DExpr, bits: u32) -> Result<DExpr, MemoryError> {
        trace!("read addr={addr:?}, bits={bits}");
        assert_eq!(addr.len(), self.ptr_size, "passed wrong sized address");

        let (addr, value) = self.resolve_address(addr)?;
        let offset = (addr - value.address) as u32 * 8;
        let val = value.bv.slice(offset, offset + bits - 1);

        trace!("Return {val:?}, value: {value:x?}");
        Ok(val)
    }

    #[tracing::instrument(skip(self))]
    fn write(&mut self, addr: &DExpr, value: DExpr) -> Result<(), MemoryError> {
        trace!("write addr={addr:?}, len={}, value={value:?}", value.len());
        assert_eq!(addr.len(), self.ptr_size, "passed wrong sized address");

        let (addr, val) = self.resolve_address_mut(addr)?;
        let offset = (addr - val.address) * 8;

        if value.len() == val.size as u32 {
            val.bv = value;
        } else {
            val.bv = val.bv.replace_part(offset as u32, value);
        }

        Ok(())
    }

    fn resolve_addresses(
        &self,
        address: &DExpr,
        upper_bound: usize,
    ) -> Result<Vec<DExpr>, MemoryError> {
        // Fast path if address is a constant.
        if let Some(_) = address.get_constant() {
            return Ok(vec![address.clone()]);
        }

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
        Ok(addresses)
    }
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

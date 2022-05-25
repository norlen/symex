/// Allocations and backing memory store.
///
/// Memory keeps track of all allocations and provides the backing memory store. All allocations
/// are tagged with an id. This allows checking for out of bounds reads.
///
/// With `null_detection` enabled this allows to to check for the possibility of the address being
/// null.
#[derive(Debug, Clone)]
pub struct ArrayMemory {
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

impl ArrayMemory {
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

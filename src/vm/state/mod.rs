use llvm_ir::{
    instruction::{self, HasResult},
    terminator,
    types::Typed,
    Function, Module, Name, Type, TypeRef,
};
use log::{debug, warn};

use super::{Allocation, AllocationType, Globals, Result};
use crate::{
    memory::NewMemory,
    project::Project,
    traits::{const_to_symbol, operand_to_symbol, Op},
    {Solver, BV},
};

mod location;
mod varmap;

pub use location::*;
pub use varmap::*;

#[derive(Debug, Clone)]
pub enum Call<'a> {
    Call(&'a instruction::Call),
    Invoke(&'a terminator::Invoke),
}

#[derive(Debug, Clone)]
pub struct Callsite<'a> {
    pub location: Location<'a>,
    pub instruction: Call<'a>,
}

impl<'a> Callsite<'a> {
    pub fn from_call(location: Location<'a>, call: &'a instruction::Call) -> Self {
        Self {
            location,
            instruction: Call::Call(call),
        }
    }

    pub fn from_invoke(location: Location<'a>, invoke: &'a terminator::Invoke) -> Self {
        Self {
            location,
            instruction: Call::Invoke(invoke),
        }
    }
}

/// A `Path` represents a single path of execution through a program. The path is composed by the
/// current execution state (`State`) and an optional constraint that will be asserted when this
/// path begins exectuting.
///
/// A single path may produce multiple other paths when encountering branching paths of execution.
#[derive(Debug, Clone)]
pub struct Path<'a> {
    /// The state to use when resuming execution.
    ///
    /// The location in the state should be where to resume execution at.
    pub state: State<'a>,

    /// Constraint to add before starting execution on this path.
    pub constraint: Option<BV>,
}

impl<'a> Path<'a> {
    /// Creates a new `Path` that resumes execution from the current `State` with no additional
    /// constraints.
    ///
    /// This should be used for the initial path in the program. When no constraints have been added
    /// to the initial function. And that it should start executing at the state's current location.
    pub fn new(state: State<'a>) -> Self {
        Self {
            state,
            constraint: None,
        }
    }

    /// Creates a new `Location` from a given state.
    ///
    /// The passed `Location` will replace the one in the state, so execution resumes at that
    /// location.
    ///
    /// The constraint will be added before resuming excution.
    pub fn new_with_constraint(
        state: State<'a>,
        location: Location<'a>,
        constraint: Option<BV>,
    ) -> Self {
        let mut state = state;
        state.current_loc = location;

        Self { state, constraint }
    }
}

/// The current state of the execution. This is the state per path of execution.
///
#[derive(Debug, Clone)]
pub struct State<'a> {
    /// The project where this state executes over.
    pub project: &'a Project,

    /// Reference to the solver, used in the `VM` as well.
    pub solver: Solver,
    pub callstack: Vec<Callsite<'a>>,

    /// Current location where we are exucting at.
    pub current_loc: Location<'a>,

    /// All defined variables. These can be pointers to memory or a register variable.
    pub vars: VarMap,

    /// Global memory.
    pub mem: NewMemory,
    pub globals: Globals<'a>,

    /// Lookup for all the variables that have been explicitly marked as `symbolic`.
    pub symbols: Vec<(String, BV)>,
}

impl<'a> State<'a> {
    pub fn new(
        project: &'a Project,
        module: &'a Module,
        function: &'a Function,
        solver: Solver,
    ) -> Self {
        let mut state = Self {
            project,
            current_loc: Location::new(module, function),
            vars: VarMap::new(10),
            mem: NewMemory::new(solver.clone(), project.ptr_size as u32),
            solver,
            callstack: Vec::new(),
            globals: Globals::new(),
            symbols: Vec::new(),
        };

        state.allocate_globals(project.modules).unwrap();
        state
    }

    pub fn get_var<'b, T>(&mut self, op: T) -> Result<BV>
    where
        T: Into<Op<'b>>,
    {
        match op.into() {
            Op::Operand(operand) => operand_to_symbol(self, operand),
            Op::Constant(constant) => const_to_symbol(self, constant),
        }
    }

    pub fn allocate(&mut self, allocation_size: u64, align: u64) -> Result<u64> {
        let align = if align == 0 {
            warn!("Alignment of 0");
            self.project.default_alignment
        } else {
            align
        };

        let addr = self.mem.allocate(allocation_size, align)?;
        Ok(addr)
    }

    /// Allocate an unitialized value `name` on the stack with size `allocation_size`.
    pub fn stack_alloc(&mut self, allocation_size: u64, align: u64) -> Result<BV> {
        let align = if align == 0 {
            warn!("Alignment of 0");
            self.project.default_alignment
        } else {
            align
        };

        let addr = self.mem.allocate(allocation_size, align)?;
        let bv = self.solver.bv_from_u64(addr, self.project.ptr_size as u32);
        Ok(bv)
    }

    // -------------------------------------------------------------------------
    // BV Helpers
    // -------------------------------------------------------------------------

    pub fn assign_bv(&mut self, name: Name, bv: BV) -> Result<()> {
        self.vars.insert(name, bv).unwrap();
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Globals
    // -------------------------------------------------------------------------

    pub fn get_global(&self, name: &Name) -> Option<&Allocation<'a>> {
        self.globals.get(name, self.current_loc.module)
    }

    // -------------------------------------------------------------------------
    // Helpers I may need, check if these should be in State.
    // -------------------------------------------------------------------------

    pub fn get_result<T: Typed + HasResult>(&self, t: &T) -> (Name, u64) {
        let name = t.get_result().clone();
        let ty = self.type_of(t);
        let size = self.project.bit_size(&ty).unwrap();
        (name, size as u64)
    }

    pub fn type_of<T: Typed>(&self, t: &T) -> TypeRef {
        self.current_loc.module.type_of(t)
    }

    fn allocate_globals(&mut self, modules: &'static [Module]) -> Result<()> {
        for module in modules {
            for var in &module.global_vars {
                // All declaration have initiaizers, so skip over definitions.
                if var.initializer.is_none() {
                    continue;
                }

                // All global variable should be a pointer.
                if let Type::PointerType { pointee_type, .. } = var.ty.as_ref() {
                    // TODO:
                    // If a variable has `unnamed_addr` the address is not significant, so we can
                    // skip allocating an address for those.
                    //
                    // For `local_unnamed_addr` the address is not significant in *that* module. To
                    // be safe, allocate addresses for those.
                    let size = {
                        // TODO: Can the types not have a size here?
                        let size = self.project.bit_size(pointee_type)?;

                        // TODO: How to handle zero sized allocations?
                        if size == 0 {
                            8
                        } else {
                            size
                        }
                    };

                    let addr = self.allocate(size as u64, var.alignment as u64)?;

                    debug!("Add GLOBAL_VARIABLE: {}", var.name);
                    self.globals.add_global_variable(var, module, addr);
                }
            }

            for function in &module.functions {
                let addr = self.allocate(self.project.ptr_size, 4)?;

                debug!("Add FUNCTION: {}", function.name);
                self.globals.add_function(function, module, addr);
            }
        }

        let current_globals = self.globals.clone();
        // Initialize all the global variables.
        for private_globals in current_globals.private_globals.values() {
            for allocation in private_globals.values() {
                self.initalize_global_variable(allocation);
            }
        }
        for allocation in current_globals.globals.values() {
            self.initalize_global_variable(allocation);
        }

        Ok(())
    }

    fn initalize_global_variable(&mut self, allocation: &Allocation<'_>) {
        if let AllocationType::Variable(var) = &allocation.kind {
            let initializer = var.initializer.clone().unwrap();
            //println!("var: {}", var.name);

            // Extremely temporary.
            match self.get_var(&initializer) {
                Ok(value) => {
                    //println!("var: {}, value: {:?}", var.name, value);

                    let bv = self
                        .solver
                        .bv_from_u64(allocation.addr, self.project.ptr_size as u32);

                    self.mem.write(&bv, value).unwrap();
                }
                Err(err) => {
                    //println!("err: {:?}", e);
                    warn!("Initialize global err: {:?}", err);
                }
            }
        }
    }
}

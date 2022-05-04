use llvm_ir::{
    instruction::{self, HasResult},
    terminator,
    types::Typed,
    Function, Name, TypeRef,
};
use log::warn;

use super::{GlobalReference, GlobalReferenceKind, GlobalReferences, Result};
use crate::{
    common::{const_to_symbol, operand_to_symbol, Op, SolutionVariable},
    memory::Memory,
    project::{ModuleHandle, Project},
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
/// path begins executing.
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
    /// The constraint will be added before resuming execution.
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

    /// Current location where we are executing at.
    pub current_loc: Location<'a>,

    /// All defined variables. These can be pointers to memory or a register variable.
    pub vars: VarMap,

    /// Global memory.
    pub mem: Memory,

    /// Lookup for all the variables that have been explicitly marked as `symbolic`.
    pub symbols: Vec<SolutionVariable>,

    /// Global references, these can be either a [Function] or a [GlobalVariable].
    ///
    /// This holds the mapping between the name of the global reference and its address.
    pub global_references: GlobalReferences<'a>,
}

impl<'a> State<'a> {
    pub fn new(
        project: &'a Project,
        module: ModuleHandle,
        function: &'a Function,
        solver: Solver,
    ) -> Self {
        let mut memory = Memory::new(solver.clone(), project.ptr_size);
        let global_references = GlobalReferences::from_project(project, &mut memory).unwrap();

        let mut state = Self {
            project,
            current_loc: Location::new(module, function),
            vars: VarMap::new(10),
            mem: memory,
            solver,
            callstack: Vec::new(),
            symbols: Vec::new(),
            global_references,
        };

        state.initialize_global_references().unwrap();
        state
    }

    pub fn get_var<'b, T>(&self, op: T) -> Result<BV>
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
            self.project.default_alignment as u64
        } else {
            align
        };

        let addr = self.mem.allocate(allocation_size, align)?;
        Ok(addr)
    }

    /// Allocate an uninitialized value `name` on the stack with size `allocation_size`.
    pub fn stack_alloc(&mut self, allocation_size: u64, align: u64) -> Result<BV> {
        let align = if align == 0 {
            warn!("Alignment of 0");
            self.project.default_alignment as u64
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

    pub fn get_global_reference(&self, name: &Name) -> Option<&GlobalReference<'a>> {
        self.global_references.get(name, self.current_loc.module)
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
        self.project.type_of(t, self.current_loc.module)
    }

    fn initialize_global_references(&mut self) -> Result<()> {
        let public_globals = self.global_references.global_references.values();
        let private_globals = self
            .global_references
            .private_global_references
            .values()
            .flat_map(|m| m.values());

        for global in public_globals.chain(private_globals) {
            if let GlobalReferenceKind::GlobalVariable(var) = global.kind {
                if let Some(initializer) = &var.initializer {
                    match self.get_var(initializer) {
                        Ok(value) => {
                            let addr = self.solver.bv_from_u64(global.addr, self.project.ptr_size);
                            self.mem.write(&addr, value)?;
                        }
                        Err(err) => {
                            warn!("Error initializing global: {:?}", err);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

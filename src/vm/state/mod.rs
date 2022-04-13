use llvm_ir::{
    instruction::{self, HasResult},
    terminator,
    types::Typed,
    Function, Module, Name, TypeRef,
};
use log::warn;

use super::{Allocation, Globals, Result};
use crate::{
    memory::bump_allocator::BumpAllocator,
    memory::simple_memory::Memory,
    project::Project,
    traits::{Op, Size, ToBV},
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

/// A `Path` represents a single path of execution through a program. The path
/// is composed by the current execution state (`State`) and an optional
/// constraint that will be asserted when this path begins exectuting.
///
/// A single path may produce multiple other paths when encountering branching
/// paths of execution.
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
    /// Creates a new `Path` that resumes execution from the current `State`
    /// with no additional constraints.
    ///
    /// This should be used for the initial path in the program. When no
    /// constraints have been added to the initial function. And that it should
    /// start executing at the state's current location.
    pub fn new(state: State<'a>) -> Self {
        Self {
            state,
            constraint: None,
        }
    }

    /// Creates a new `Location` from a given state.
    ///
    /// The passed `Location` will replace the one in the state, so execution
    /// resumes at that location.
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

    /// Stack allocations.
    pub stack: BumpAllocator,

    pub callstack: Vec<Callsite<'a>>,

    /// Current location where we are exucting at.
    pub current_loc: Location<'a>,

    /// All defined variables. These can be pointers to memory or a register
    /// variable.
    pub vars: VarMap,

    /// The global memory. That both stack and heap allocations use.
    pub mem: Memory,

    pub globals: Globals<'a>,
}

impl<'a> State<'a> {
    pub fn new(
        project: &'a Project,
        module: &'a Module,
        function: &'a Function,
        solver: Solver,
    ) -> Self {
        Self {
            stack: BumpAllocator::new(),
            project,
            current_loc: Location::new(module, function),
            vars: VarMap::new(10),
            mem: Memory::new_uninitialized(solver.clone(), project.ptr_size as u32),
            solver,
            callstack: Vec::new(),
            globals: Globals::new(),
        }
    }

    pub fn get_var<'b, T>(&mut self, op: T) -> Result<BV>
    where
        T: Into<Op<'b>>,
    {
        match op.into() {
            Op::Operand(op) => op.to_bv(self),
            Op::Constant(c) => c.to_bv(self),
        }
    }

    /// Allocate an unitialized value `name` on the stack with size `allocation_size`.
    pub fn stack_alloc(&mut self, allocation_size: u64, align: u64) -> Result<BV> {
        let align = if align == 0 {
            warn!("Alignment of 0");
            self.project.default_alignment
        } else {
            align
        };

        let ptr = self
            .stack
            .get_address(allocation_size as usize, align as usize);

        let bv = self
            .solver
            .bv_from_u64(ptr as u64, self.project.ptr_size as u32);
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

    // pub fn initialize_globals(&mut self) {
    //     for (_, allocation) in &self.globals.globals {
    //         match &allocation.kind {
    //             AllocationType::Variable(v) => {
    //                 let value = self.get_bv_from_constant(&v.initializer).unwrap();
    //                 self.mem.write(&allocation.addr_bv, value).unwrap();
    //             }
    //             AllocationType::Function(_) => {}
    //         }
    //     }
    //     for (_, map) in &self.globals.private_globals {
    //         for (_, allocation) in map {
    //             match &allocation.kind {
    //                 AllocationType::Variable(v) => {
    //                     let value = self.get_bv_from_constant(&v.initializer).unwrap();
    //                     self.mem.write(&allocation.addr_bv, value).unwrap();
    //                 }
    //                 AllocationType::Function(_) => {}
    //             }
    //         }
    //     }
    // }

    // -------------------------------------------------------------------------
    // Helpers I may need, check if these should be in State.
    // -------------------------------------------------------------------------

    pub fn get_result<T: Typed + HasResult>(&self, t: &T) -> (Name, u64) {
        let name = t.get_result().clone();
        let ty = self.type_of(t);
        let size = ty.size(self.project).unwrap();
        (name, size)
    }

    pub fn type_of<T: Typed>(&self, t: &T) -> TypeRef {
        self.current_loc.module.type_of(t)
    }
}

use crate::{
    llvm::{operand_to_bv, size_in_bits},
    memory::bump_allocator::BumpAllocator,
    memory::simple_memory::Memory,
    project::Project,
    {Solver, BV},
};
use llvm_ir::{
    instruction::{self, HasResult},
    terminator,
    types::Typed,
    Constant, Function, Module, Name, Operand, TypeRef,
};

use super::varmap::VarMap;
use super::Location;
use anyhow::Result;

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

        Self {
            state,
            constraint: constraint,
        }
    }
}

/// The current state of the execution. This is the state per path of execution.
///
#[derive(Debug, Clone)]
pub struct State<'a> {
    /// The project where this state executes over.
    project: &'a Project,

    /// Reference to the solver, used in the `VM` as well.
    pub solver: Solver,

    /// Stack allocations.
    stack: BumpAllocator,

    pub callstack: Vec<Callsite<'a>>,

    /// Current location where we are exucting at.
    pub current_loc: Location<'a>,

    /// All defined variables. These can be pointers to memory or a register
    /// variable.
    pub vars: VarMap,

    /// The global memory. That both stack and heap allocations use.
    pub mem: Memory,
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
            mem: Memory::new_uninitialized(solver.clone()),
            solver,
            callstack: Vec::new(),
        }
    }

    /// Allocate an unitialized value `name` on the stack with size `allocation_size`.
    pub fn stack_alloc(&mut self, allocation_size: usize, align: usize) -> Result<BV> {
        let ptr = self.stack.get_address(allocation_size, align);

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

    pub fn get_bv_from_operand(&mut self, op: &Operand) -> Result<BV> {
        let bv = operand_to_bv(op, self).unwrap();
        Ok(bv)
    }

    pub fn get_bv_from_constant(&mut self, constant: &Constant) -> Result<BV> {
        todo!()
    }

    // -------------------------------------------------------------------------
    // Helpers I may need, check if these should be in State.
    // -------------------------------------------------------------------------

    pub fn get_result<T: Typed + HasResult>(&self, t: &T) -> (Name, usize) {
        let name = t.get_result().clone();
        let ty = self.type_of(t);
        let size = size_in_bits(&ty, self.project).unwrap(); // TODO
        (name, size)
    }

    pub fn type_of<T: Typed>(&self, t: &T) -> TypeRef {
        self.current_loc.module.type_of(t)
    }
}

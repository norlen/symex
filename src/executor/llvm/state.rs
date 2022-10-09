use llvm_ir::{types::Typed, Function, Name, TypeRef};
use std::collections::HashMap;
use tracing::warn;

// use crate::memory::ArrayMemory;
use crate::memory::ObjectMemory;

use super::{const_to_expr, operand_to_expr, GlobalReferenceKind, GlobalReferences};
use crate::{
    core::{memory::Memory, smt::SolverContext},
    executor::llvm::{
        common::Op,
        project::{ModuleHandle, Project},
    },
    smt::{DContext, DExpr, DSolver},
    Variable,
};

pub use super::{Location, Result};

/// Stack frame keeps track of information related to a specific stack frame.
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Variables created in a specific stack frame.
    pub registers: HashMap<Name, DExpr>,

    /// Location to start or resume execution at in this stack frame.
    pub location: Location,

    /// Keeps track of the number of times a basic block has been branched to.
    ///
    /// Can be used to check for exceeded iteration counts.
    pub(super) basic_block_entry_count: HashMap<Name, usize>,
}

impl StackFrame {
    pub fn new(location: Location) -> Self {
        Self {
            registers: HashMap::new(),
            location,
            basic_block_entry_count: HashMap::new(),
        }
    }
}

// // Move somewhere else
// #[derive(Debug, Clone)]
// pub struct ConstraintSet {}

#[derive(Debug, Clone)]
pub struct LLVMState {
    /// SMT Context.
    pub ctx: &'static DContext,

    /// The path condition, holds all the saved constraints.
    pub constraints: DSolver,

    /// List of variables marked as symbolic.
    pub marked_symbolic: Vec<Variable>,

    pub memory: ObjectMemory,
    // pub memory: ArrayMemory,
    pub stack_frames: Vec<StackFrame>,

    // Check if I should have this here, or maybe just pass the executor instead
    pub project: &'static Project,

    /// Global references, these can be either a [Function] or a [GlobalVariable].
    ///
    /// This holds the mapping between the name of the global reference and its address.
    pub global_references: GlobalReferences,
}

impl LLVMState {
    pub fn new(
        ctx: &'static DContext,
        project: &'static Project,
        constraints: DSolver,
        module: ModuleHandle,
        function: &'static Function,
    ) -> Self {
        // let mut memory = ArrayMemory::new(ctx.clone(), project.ptr_size);
        let mut memory = ObjectMemory::new(ctx, project.ptr_size, constraints.clone());

        let global_references = GlobalReferences::from_project(project, &mut memory).unwrap();

        let location = Location::new(module, function);
        let stack_frame = StackFrame::new(location);

        let mut state = Self {
            ctx,
            constraints,
            marked_symbolic: Vec::new(),
            memory,
            stack_frames: vec![stack_frame],
            project,
            global_references,
        };
        state.initialize_global_references().unwrap();

        state
    }

    /// Fork the current state replacing the current executing location with the passed location.
    pub fn fork(&self, location: Location) -> Self {
        let mut new_state = self.clone();
        new_state.stack_frames.last_mut().unwrap().location = location;
        new_state
    }

    /// Retrieves or creates an [Expr] from an [Operand] or [Constant].
    pub fn get_expr<'b, T>(&self, op: T) -> Result<DExpr>
    where
        T: Into<Op<'b>>,
    {
        match op.into() {
            Op::Operand(operand) => operand_to_expr(self, operand),
            Op::Constant(constant) => const_to_expr(self, constant),
        }
    }

    pub fn type_of<T: Typed>(&self, t: &T) -> TypeRef {
        let current_module = self.stack_frames.last().unwrap().location.module;
        self.project.type_of(t, current_module)
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
                    match self.get_expr(initializer) {
                        Ok(value) => {
                            let addr = self.ctx.from_u64(global.addr, self.project.ptr_size);
                            self.memory.write(&addr, value)?;
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

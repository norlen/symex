use crate::executor::llvm::common::Op;
use crate::executor::llvm::project::ModuleHandle;
use crate::executor::llvm::project::Project;
use crate::memory::Memory;
use crate::memory::ObjectMemory;
use crate::smt::DContext;
use crate::smt::DExpr;
use crate::smt::DSolver;
use crate::smt::SolverContext;
use llvm_ir::types::Typed;
use llvm_ir::Function;
use llvm_ir::Name;
use llvm_ir::TypeRef;
use std::collections::HashMap;
use tracing::warn;

use super::const_to_expr;
use super::operand_to_expr;
use super::GlobalReferenceKind;
use super::GlobalReferences;
pub use super::{Location, Result};

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub registers: HashMap<Name, DExpr>,

    pub location: Location,
}

impl StackFrame {
    pub fn new(location: Location) -> Self {
        Self {
            registers: HashMap::new(),
            location,
        }
    }
}

// // Move somewhere else
// #[derive(Debug, Clone)]
// pub struct ConstraintSet {}

#[derive(Debug, Clone)]
pub struct LLVMState {
    pub ctx: &'static DContext,

    pub constraints: DSolver,

    pub marked_symbolic: Vec<DExpr>,

    pub memory: ObjectMemory,

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
        // state.initialize_global_references().unwrap();

        state
    }

    pub fn fork(&self, location: Location) -> Self {
        let mut new_state = self.clone();
        new_state.stack_frames.last_mut().unwrap().location = location;
        new_state
    }

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
}

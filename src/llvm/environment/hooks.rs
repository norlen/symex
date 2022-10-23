//! Hooks
//!
use llvm_ir::{Name, Operand, Type};
use std::collections::HashMap;
use tracing::trace;

// These should be moved out of LLVM and be made general purpose enough to be used for any
// executor.
//
// This would require a general purpose project as well though.

use crate::{
    core::memory::{Memory, BITS_IN_BYTE},
    core::smt::{Expression, Solver, SolverContext, SolverError},
    llvm::{
        common::type_to_expr_type,
        executor::{LLVMExecutor, ReturnValue},
        LLVMExecutorError,
    },
    util::{ExpressionType, Variable},
};

/// Hook type
pub type Hook = fn(&mut LLVMExecutor<'_>, &[&Operand]) -> Result<ReturnValue, LLVMExecutorError>;

pub struct Hooks {
    hooks: HashMap<String, Hook>,
}

impl Default for Hooks {
    fn default() -> Self {
        Self::new()
    }
}

impl Hooks {
    pub fn new() -> Self {
        let mut hooks = Self {
            hooks: HashMap::new(),
        };

        hooks.add("symex_lib::assume", assume);
        hooks.add("symex_lib::symbolic", symbolic);
        hooks.add("symex_lib::ignore_path", ignore);

        // These are not mangled, so these can be called from e.g. C.
        hooks.add("symex_assume", assume);
        hooks.add("symex_symbolic", symbolic_no_type);

        hooks
    }

    fn add(&mut self, name: impl Into<String>, hook: Hook) {
        self.hooks.insert(name.into(), hook);
    }

    pub fn get(&self, name: &str) -> Option<Hook> {
        trace!("hooks: get {}", name);
        self.hooks.get(name).copied()
    }
}

pub fn ignore(
    _vm: &mut LLVMExecutor<'_>,
    _args: &[&Operand],
) -> Result<ReturnValue, LLVMExecutorError> {
    Err(LLVMExecutorError::SuppressPath)
}

pub fn assume(
    vm: &mut LLVMExecutor<'_>,
    args: &[&Operand],
) -> Result<ReturnValue, LLVMExecutorError> {
    trace!("assume info: {:?}", args);

    let condition = vm.state.get_expr(args[0])?;
    match condition.len() {
        // Boolean condition.
        1 => {
            vm.state.constraints.assert(&condition);
        }
        // Otherwise, check for non zero.
        _ => {
            let zero = vm.state.ctx.zero(condition.len());
            let condition = condition._ne(&zero);
            vm.state.constraints.assert(&condition)
        }
    }

    if vm.state.constraints.is_sat()? {
        Ok(ReturnValue::Void)
    } else {
        Err(LLVMExecutorError::SolverError(SolverError::Unsat))
    }
}

pub fn symbolic_no_type(
    vm: &mut LLVMExecutor<'_>,
    args: &[&Operand],
) -> Result<ReturnValue, LLVMExecutorError> {
    trace!("symbolic_no_type args: {:?}", args);

    let addr = args[0];
    let addr_ty = vm.state.type_of(addr);
    if !matches!(addr_ty.as_ref(), Type::PointerType { .. }) {
        panic!("Expected pointer type");
    }

    let size = vm.state.get_expr(args[1])?;

    let concrete_size_in_bits = match size.get_constant() {
        Some(value) => value as u32 * BITS_IN_BYTE,
        None => panic!("Size for symbolic requires constant size"),
    };

    let name = get_operand_name(&addr);
    let new_value = vm.state.ctx.unconstrained(concrete_size_in_bits, &name);

    let addr = vm.state.get_expr(addr)?;
    vm.state.memory.write(&addr, new_value.clone())?;

    let var = Variable {
        name: None,
        value: new_value,
        ty: ExpressionType::Unknown,
    };
    vm.state.marked_symbolic.push(var);

    Ok(ReturnValue::Void)
}

pub fn symbolic(
    vm: &mut LLVMExecutor<'_>,
    args: &[&Operand],
) -> Result<ReturnValue, LLVMExecutorError> {
    trace!("symbolic args: {:?}", args);

    let addr = args[0];
    let addr_ty = vm.state.type_of(addr);
    if let Type::PointerType {
        pointee_type: inner_ty,
        ..
    } = addr_ty.as_ref()
    {
        let size = vm.project.bit_size(inner_ty.as_ref())?;
        let name = get_operand_name(&addr);
        let new_value = vm.state.ctx.unconstrained(size, &name);

        let var = Variable {
            name: Some(name),
            value: new_value.clone(),
            ty: type_to_expr_type(inner_ty.as_ref(), vm.project),
        };
        vm.state.marked_symbolic.push(var);

        let addr = vm.state.get_expr(addr)?;
        vm.state.memory.write(&addr, new_value)?;

        Ok(ReturnValue::Void)
    } else {
        panic!("not a pointer type");
    }
}

fn get_operand_name(op: &Operand) -> String {
    let name = match op {
        Operand::LocalOperand { name, .. } => match name {
            Name::Name(name) => String::from(name.as_str()),
            Name::Number(_) => name.to_string(),
        },
        Operand::ConstantOperand(_) => op.to_string(),
        Operand::MetadataOperand => op.to_string(),
    };

    // Add random number at the end, this is to give each call to symbolic a unique name. Even if
    // its called for variables with the same name.
    format!("{}-{}", name, rand::random::<u32>())
}

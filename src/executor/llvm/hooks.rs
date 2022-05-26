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
    executor::llvm::{LLVMExecutorError, ReturnValue},
    memory::Memory,
    smt::{Expression, Solutions, Solver, SolverContext},
    LLVMExecutor,
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

        hooks.add("x0001e_lib::assume", assume);
        hooks.add("x0001e_lib::symbolic", symbolic);
        hooks.add("assume", assume);
        hooks.add("symbolic", symbolic_no_type);

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
        panic!("unsat continue");
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

    let values = vm.state.constraints.get_values(&size, 1)?;
    let concrete_size = match values {
        Solutions::Exactly(values) => values[0].get_constant().unwrap(),
        Solutions::AtLeast(_) => panic!("Multiple solutions for size of variable not supported"),
    };

    let name = get_operand_name(&addr);
    let new_value = vm.state.ctx.unconstrained(concrete_size as u32, &name);

    let addr = vm.state.get_expr(addr)?;
    vm.state.memory.write(&addr, new_value)?;

    // let solution_var = SolutionVariable {
    //     name,
    //     value,
    //     ty: None,
    // };
    // vm.state.symbols.push(solution_var);

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

        // let solution_var = SolutionVariable {
        //     name,
        //     value: new_symbol.clone(),
        //     ty: Some(inner_ty.clone()),
        // };
        // vm.state.symbols.push(solution_var);

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

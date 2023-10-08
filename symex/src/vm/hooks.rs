//! Hooks
//!
use llvm_ir::Value;
use std::collections::HashMap;
use tracing::{debug, trace};

// These should be moved out of LLVM and be made general purpose enough to be used for any
// executor.
//
// This would require a general purpose project as well though.

use crate::{
    memory::BITS_IN_BYTE,
    smt::SolverError,
    util::{ExpressionType, Variable},
    vm::{executor::LLVMExecutor, AnalysisError, LLVMExecutorError},
};

use super::PathResult;

/// Hook type
pub type Hook = fn(&mut LLVMExecutor<'_>, &[Value]) -> Result<PathResult, LLVMExecutorError>;

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

        hooks.add("__rust_alloc", rust_alloc);
        hooks.add("__rust_dealloc", rust_dealloc);
        hooks.add("__rust_realloc", rust_realloc);
        hooks.add("__rust_alloc_zeroed", rust_alloc_zeroed);
        hooks.add("std::process::exit", exit);
        hooks.add("core::panicking::panic_bounds_check", abort);
        hooks.add("core::panicking::panic", abort);
        hooks.add("core::panicking::panic_fmt", abort);

        hooks
    }

    fn add(&mut self, name: impl Into<String>, hook: Hook) {
        self.hooks.insert(name.into(), hook);
    }

    pub fn get(&self, name: &str) -> Option<Hook> {
        self.hooks.get(name).copied()
    }
}

pub fn ignore(
    _vm: &mut LLVMExecutor<'_>,
    _args: &[Value],
) -> Result<PathResult, LLVMExecutorError> {
    Ok(PathResult::Suppress)
}

pub fn assume(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult, LLVMExecutorError> {
    trace!("assume info: {:?}", args);

    let condition = vm.state.get_expr(&args[0])?;
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
        Ok(PathResult::Success(None))
    } else {
        Err(LLVMExecutorError::SolverError(SolverError::Unsat))
    }
}

pub fn symbolic_no_type(
    vm: &mut LLVMExecutor<'_>,
    args: &[Value],
) -> Result<PathResult, LLVMExecutorError> {
    trace!("symbolic_no_type args: {:?}", args);

    let addr = &args[0];
    // let addr_ty = vm.state.type_of(addr);
    // if !matches!(addr_ty.as_ref(), Type::PointerType { .. }) {
    //     panic!("Expected pointer type");
    // }

    let size = vm.state.get_expr(&args[1])?;

    let concrete_size_in_bits = match size.get_constant() {
        Some(value) => value as u32 * BITS_IN_BYTE,
        None => panic!("Size for symbolic requires constant size"),
    };

    let name = get_operand_name(addr);
    let new_value = vm.state.ctx.unconstrained(concrete_size_in_bits, &name);

    let addr = vm.state.get_expr(addr)?;
    vm.state.memory.write(&addr, new_value.clone())?;

    let var = Variable {
        name: None,
        value: new_value,
        ty: ExpressionType::Unknown,
    };
    vm.state.marked_symbolic.push(var);

    Ok(PathResult::Success(None))
}

pub fn symbolic(
    vm: &mut LLVMExecutor<'_>,
    args: &[Value],
) -> Result<PathResult, LLVMExecutorError> {
    let addr = &args[0];
    trace!("call symbolic({})", addr);

    if addr.ty().is_pointer() {
        // TODO: We need the size of the pointed to value, which we cannot easily get with
        // opaque pointers.
        let size = {
            // Read the pointed to object from memory and get the size from there, not entirely
            // sure this works for all cases... Since, I think we may sometimes only want
            // part of the memory object to be reset to entirely symbolic.
            let addr = vm.state.get_expr(addr)?;
            let addr = addr.get_constant().expect("expected constant addr");
            let obj = vm
                .state
                .memory
                .get_object(addr)
                .expect("coult not find object");
            obj.bit_size()
        };

        // let size = vm.project.bit_size(inner_ty.as_ref())?;
        let name = get_operand_name(addr);
        let new_value = vm.state.ctx.unconstrained(size as u32, &name);

        let var = Variable {
            name: Some(name),
            value: new_value.clone(),
            // ty: type_to_expr_type(inner_ty.as_ref(), vm.project),
            ty: ExpressionType::Unknown,
        };
        vm.state.marked_symbolic.push(var);

        let addr = vm.state.get_expr(addr)?;
        vm.state.memory.write(&addr, new_value)?;

        Ok(PathResult::Success(None))
    } else {
        panic!("not a pointer type");
    }
}

fn get_operand_name(_op: &Value) -> String {
    // let name = (op);
    // let name = if name.is_empty() {
    //     "symex".to_string()
    // } else {
    //     name
    // };
    let name = "name-todo";

    // Add random number at the end, this is to give each call to symbolic a unique name. Even if
    // its called for variables with the same name.
    format!("{}-{}", name, rand::random::<u32>())
}

// Temporary function to get a single u64 value.
//
// Will not work if the expression can hold multiple values.
fn get_single_u64_from_op(vm: &mut LLVMExecutor<'_>, op: &Value) -> Result<u64, LLVMExecutorError> {
    let expr = vm.state.get_expr(op)?;
    let value = vm
        .state
        .constraints
        .get_value(&expr)?
        .get_constant()
        .unwrap();
    Ok(value)
}

fn exit(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult, LLVMExecutorError> {
    let exit_code = vm.state.get_expr(&args[0])?;
    let exit_code = exit_code.get_constant().unwrap();
    Err(LLVMExecutorError::Abort(exit_code as i64))
}

/// Hook that tells the VM to abort.
pub fn abort(_vm: &mut LLVMExecutor<'_>, _args: &[Value]) -> Result<PathResult, LLVMExecutorError> {
    debug!("Hook: panic!");
    Ok(PathResult::Failure(AnalysisError::Panic))
}

// fn __rust_alloc(size: usize, align: usize) -> *mut u8;
fn rust_alloc(vm: &mut LLVMExecutor<'_>, args: &[Value]) -> Result<PathResult, LLVMExecutorError> {
    assert_eq!(args.len(), 2);

    let size_in_bytes = get_single_u64_from_op(vm, &args[0])?;
    let size_in_bits = size_in_bytes * BITS_IN_BYTE as u64;

    let align = get_single_u64_from_op(vm, &args[1])?;

    let addr = vm.state.memory.allocate(size_in_bits, align)?;
    let addr = vm.state.ctx.from_u64(addr, vm.project.ptr_size);

    Ok(PathResult::Success(Some(addr)))
}

// fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize);
fn rust_dealloc(
    _vm: &mut LLVMExecutor<'_>,
    _args: &[Value],
) -> Result<PathResult, LLVMExecutorError> {
    // Currently, de-allocating is not supported.
    Ok(PathResult::Success(None))
}

// fn __rust_realloc(ptr: *mut u8, old_size: usize, align: usize, new_size: usize) -> *mut u8;
fn rust_realloc(
    vm: &mut LLVMExecutor<'_>,
    args: &[Value],
) -> Result<PathResult, LLVMExecutorError> {
    assert_eq!(args.len(), 4);

    let addr = vm.state.get_expr(&args[0])?;
    let size = get_single_u64_from_op(vm, &args[1])?;
    let align = get_single_u64_from_op(vm, &args[2])?;
    let size_in_bytes = get_single_u64_from_op(vm, &args[3])?;
    let size_in_bits = size_in_bytes * BITS_IN_BYTE as u64;

    let new_addr = vm.state.memory.allocate(size_in_bits, align)?;
    let new_addr = vm.state.ctx.from_u64(new_addr, vm.project.ptr_size);

    let old_data = vm.state.memory.read(&addr, size as u32)?;
    vm.state.memory.write(&new_addr, old_data)?;

    Ok(PathResult::Success(Some(new_addr)))
}

// fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8;
fn rust_alloc_zeroed(
    vm: &mut LLVMExecutor<'_>,
    args: &[Value],
) -> Result<PathResult, LLVMExecutorError> {
    assert_eq!(args.len(), 2);

    let size_in_bytes = get_single_u64_from_op(vm, &args[0])?;
    let size_in_bits = size_in_bytes * BITS_IN_BYTE as u64;

    let align = get_single_u64_from_op(vm, &args[1])?;

    let addr = vm.state.memory.allocate(size_in_bits, align)?;
    let addr = vm.state.ctx.from_u64(addr, vm.project.ptr_size);

    let zeroes = vm.state.ctx.zero(size_in_bits as u32);
    vm.state.memory.write(&addr, zeroes)?;

    Ok(PathResult::Success(Some(addr)))
}

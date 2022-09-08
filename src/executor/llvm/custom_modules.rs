use llvm_ir::Operand;
use tracing::debug;

use crate::{
    core::memory::{Memory, BITS_IN_BYTE},
    core::smt::{Expression, SolverContext},
    executor::llvm::{LLVMExecutorError, ReturnValue},
    LLVMExecutor,
};

type UserDefinedFunction = (
    &'static str,
    fn(&mut LLVMExecutor<'_>, &[&Operand]) -> Result<ReturnValue, LLVMExecutorError>,
);

pub trait CustomModule {
    fn get_name(&self) -> &'static str;

    fn get_all_functions(&self) -> &[UserDefinedFunction];
}

/// Custom module for Rust functionality.
///
/// Includes allocation functions from
/// https://github.com/rust-lang/rust/blob/master/library/alloc/src/alloc.rs
pub struct RustModule {}

impl CustomModule for RustModule {
    fn get_name(&self) -> &'static str {
        "rust"
    }

    fn get_all_functions(&self) -> &[UserDefinedFunction] {
        &[
            ("__rust_alloc", rust_alloc),
            ("__rust_dealloc", rust_dealloc),
            ("__rust_realloc", rust_realloc),
            ("__rust_alloc_zeroed", rust_alloc_zeroed),
            ("core::panicking::panic_bounds_check", abort),
            ("core::panicking::panic", abort),
            ("core::panicking::panic_fmt", abort),
        ]
    }
}

// Temporary function to get a single u64 value.
//
// Will not work if the expression can hold multiple values.
fn get_single_u64_from_op(
    vm: &mut LLVMExecutor<'_>,
    op: &Operand,
) -> Result<u64, LLVMExecutorError> {
    let expr = vm.state.get_expr(op)?;
    let value = vm
        .state
        .constraints
        .get_value(&expr)?
        .get_constant()
        .unwrap();
    Ok(value)
}

/// Hook that tells the VM to abort.
pub fn abort(
    _vm: &mut LLVMExecutor<'_>,
    _args: &[&Operand],
) -> Result<ReturnValue, LLVMExecutorError> {
    debug!("Hook: ABORT");
    Err(LLVMExecutorError::Abort(-1))
}

// fn __rust_alloc(size: usize, align: usize) -> *mut u8;
fn rust_alloc(
    vm: &mut LLVMExecutor<'_>,
    args: &[&Operand],
) -> Result<ReturnValue, LLVMExecutorError> {
    assert_eq!(args.len(), 2);

    let size_in_bytes = get_single_u64_from_op(vm, args[0])?;
    let size_in_bits = size_in_bytes * BITS_IN_BYTE as u64;

    let align = get_single_u64_from_op(vm, args[1])?;

    let addr = vm.state.memory.allocate(size_in_bits, align)?;
    let addr = vm.state.ctx.from_u64(addr, vm.project.ptr_size);

    Ok(ReturnValue::Value(addr))
}

// fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize);
fn rust_dealloc(
    _vm: &mut LLVMExecutor<'_>,
    _args: &[&Operand],
) -> Result<ReturnValue, LLVMExecutorError> {
    // Currently, de-allocating is not supported.
    Ok(ReturnValue::Void)
}

// fn __rust_realloc(ptr: *mut u8, old_size: usize, align: usize, new_size: usize) -> *mut u8;
fn rust_realloc(
    vm: &mut LLVMExecutor<'_>,
    args: &[&Operand],
) -> Result<ReturnValue, LLVMExecutorError> {
    assert_eq!(args.len(), 4);

    let addr = vm.state.get_expr(args[0])?;
    let size = get_single_u64_from_op(vm, args[1])?;
    let align = get_single_u64_from_op(vm, args[2])?;
    let size_in_bytes = get_single_u64_from_op(vm, args[3])?;
    let size_in_bits = size_in_bytes * BITS_IN_BYTE as u64;

    let new_addr = vm.state.memory.allocate(size_in_bits, align)?;
    let new_addr = vm.state.ctx.from_u64(new_addr, vm.project.ptr_size);

    let old_data = vm.state.memory.read(&addr, size as u32)?;
    vm.state.memory.write(&new_addr, old_data)?;

    Ok(ReturnValue::Value(new_addr))
}

// fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8;
fn rust_alloc_zeroed(
    vm: &mut LLVMExecutor<'_>,
    args: &[&Operand],
) -> Result<ReturnValue, LLVMExecutorError> {
    assert_eq!(args.len(), 2);

    let size_in_bytes = get_single_u64_from_op(vm, args[0])?;
    let size_in_bits = size_in_bytes * BITS_IN_BYTE as u64;

    let align = get_single_u64_from_op(vm, args[1])?;

    let addr = vm.state.memory.allocate(size_in_bits, align)?;
    let addr = vm.state.ctx.from_u64(addr, vm.project.ptr_size);

    let zeroes = vm.state.ctx.zero(size_in_bits as u32);
    vm.state.memory.write(&addr, zeroes)?;

    Ok(ReturnValue::Value(addr))
}

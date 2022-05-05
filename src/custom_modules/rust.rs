use llvm_ir::Operand;
use log::debug;

use super::{CustomModule, UserDefinedFunction};
use crate::{
    common::ToValue,
    hooks::FnInfo,
    memory::BITS_IN_BYTE,
    vm::{Result, ReturnValue, VMError, VM},
    Solutions,
};

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

/// Hook that tells the VM to abort.
pub fn abort(_vm: &mut VM<'_>, _info: FnInfo) -> Result<ReturnValue> {
    debug!("Hook: ABORT");
    Err(VMError::Abort(-1))
}

fn rust_alloc(vm: &mut VM<'_>, info: FnInfo) -> Result<ReturnValue> {
    // fn __rust_alloc(size: usize, align: usize) -> *mut u8;
    assert_eq!(info.arguments.len(), 2);

    let size_in_bytes = &info.arguments[0].0;
    let size_in_bytes = match size_in_bytes {
        Operand::LocalOperand { .. } => {
            let size = vm.state.get_var(size_in_bytes)?;
            vm.solver.get_solution_maximum(&size)?
        }
        Operand::ConstantOperand(_) => size_in_bytes.to_value()?,
        Operand::MetadataOperand => todo!(),
    };
    let size_in_bits = size_in_bytes * BITS_IN_BYTE as u64;

    let align = &info.arguments[1].0;
    let align = match align {
        Operand::LocalOperand { .. } => {
            let size = vm.state.get_var(align)?;
            vm.solver.get_solution_maximum(&size)?
        }
        Operand::ConstantOperand(_) => align.to_value()?,
        Operand::MetadataOperand => todo!(),
    };

    let addr = vm.state.allocate(size_in_bits, align)?;
    let addr = vm.solver.bv_from_u64(addr, vm.project.ptr_size);

    Ok(ReturnValue::Value(addr))
}

fn rust_dealloc(_vm: &mut VM<'_>, _info: FnInfo) -> Result<ReturnValue> {
    // fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize);

    // Currently, de-allocating is not supported.
    Ok(ReturnValue::Void)
}

fn rust_realloc(vm: &mut VM<'_>, info: FnInfo) -> Result<ReturnValue> {
    // fn __rust_realloc(ptr: *mut u8, old_size: usize, align: usize, new_size: usize) -> *mut u8;
    assert_eq!(info.arguments.len(), 4);

    let addr = &info.arguments[0].0;
    let addr = vm.state.get_var(addr)?;

    let size = &info.arguments[1].0;
    let size = vm.state.get_var(size)?;

    let size = match vm.solver.get_solutions_for_bv(&size, 1)? {
        Solutions::None => todo!(),
        Solutions::Exactly(values) => values[0].as_u64().unwrap(),
        Solutions::AtLeast(_) => todo!(),
    };

    let align = &info.arguments[2].0;
    let align = match align {
        Operand::LocalOperand { .. } => {
            let size = vm.state.get_var(align)?;
            vm.solver.get_solution_maximum(&size)?
        }
        Operand::ConstantOperand(_) => align.to_value()?,
        Operand::MetadataOperand => todo!(),
    };

    let size_in_bytes = &info.arguments[3].0;
    let size_in_bytes = match size_in_bytes {
        Operand::LocalOperand { .. } => {
            let size = vm.state.get_var(size_in_bytes)?;
            vm.solver.get_solution_maximum(&size)?
        }
        Operand::ConstantOperand(_) => size_in_bytes.to_value()?,
        Operand::MetadataOperand => todo!(),
    };
    let size_in_bits = size_in_bytes * BITS_IN_BYTE as u64;

    let new_addr = vm.state.allocate(size_in_bits, align)?;
    let new_addr = vm.solver.bv_from_u64(new_addr, vm.project.ptr_size);

    let old_data = vm.state.mem.read(&addr, size as u32)?;
    vm.state.mem.write(&new_addr, old_data)?;

    Ok(ReturnValue::Value(new_addr))
}

fn rust_alloc_zeroed(_vm: &mut VM<'_>, _info: FnInfo) -> Result<ReturnValue> {
    // fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8;
    todo!()
}

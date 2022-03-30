use anyhow::Result;

use super::FnInfo;
use crate::vm::ReturnValue;
use crate::{llvm::operand_to_bv, vm::VM};

pub fn llvm_smul_with_overflow(vm: &mut VM, f: FnInfo) -> Result<ReturnValue> {
    assert_eq!(f.arguments.len(), 2);
    let (a0, _) = f.arguments.get(0).unwrap();
    let (a1, _) = f.arguments.get(1).unwrap();

    let a0 = operand_to_bv(a0, &mut vm.state)?;
    let a1 = operand_to_bv(a1, &mut vm.state)?;

    let result = a0.mul(&a1);
    let overflow = a0.smulo(&a1);
    assert_eq!(overflow.get_width(), 1);

    let ret = overflow.concat(&result).into();
    Ok(ReturnValue::Return(ret))
}

pub fn llvm_expect(vm: &mut VM, f: FnInfo) -> Result<ReturnValue> {
    assert_eq!(f.arguments.len(), 2);
    let (a0, _) = f.arguments.get(0).unwrap();
    let val = operand_to_bv(a0, &mut vm.state)?;

    Ok(ReturnValue::Return(val))
}

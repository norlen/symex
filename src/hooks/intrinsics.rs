use anyhow::Result;

use super::FnInfo;
use crate::vm::ReturnValue;
use crate::vm::VM;

pub fn llvm_smul_with_overflow(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    assert_eq!(f.arguments.len(), 2);
    let (a0, _) = f.arguments.get(0).unwrap();
    let (a1, _) = f.arguments.get(1).unwrap();

    let a0 = vm.state.get_bv_from_operand(a0).unwrap();
    let a1 = vm.state.get_bv_from_operand(a1).unwrap();

    let result = a0.mul(&a1);
    let overflow = a0.smulo(&a1);
    assert_eq!(overflow.get_width(), 1);

    let ret = overflow.concat(&result).into();
    Ok(ReturnValue::Return(ret))
}

pub fn llvm_expect(vm: &mut VM<'_>, f: FnInfo) -> Result<ReturnValue> {
    assert_eq!(f.arguments.len(), 2);
    let (a0, _) = f.arguments.get(0).unwrap();
    let val = vm.state.get_bv_from_operand(a0).unwrap();

    Ok(ReturnValue::Return(val))
}

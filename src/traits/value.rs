use anyhow::anyhow;
use llvm_ir::{Constant, ConstantRef, Operand};

use crate::vm::{Result, VMError};

/// Trait to get a value from [llvm_ir::Operand] or [llvm_ir::Constant].
///
/// The conversions are simple and retreives an underlying value, they do not try to handle more
/// advanced expressions.
pub trait ToValue<T> {
    /// Tries to get the concrete value.
    fn to_value(&self) -> Result<T>;
}

impl ToValue<u64> for Operand {
    fn to_value(&self) -> Result<u64> {
        match self {
            Operand::LocalOperand { .. } => Err(VMError::Other(anyhow!(
                "Cannot get value from a Local Operand"
            ))),
            Operand::ConstantOperand(constant) => constant.to_value(),
            Operand::MetadataOperand => Err(VMError::MalformedInstruction),
        }
    }
}

impl ToValue<u64> for ConstantRef {
    fn to_value(&self) -> Result<u64> {
        self.as_ref().to_value()
    }
}

impl ToValue<u64> for Constant {
    fn to_value(&self) -> Result<u64> {
        match self {
            Constant::Int { value, .. } => Ok(*value),
            _ => todo!(),
        }
    }
}

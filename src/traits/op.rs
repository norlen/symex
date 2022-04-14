use llvm_ir::{types::Typed, Constant, ConstantRef, Operand};

use super::ToValue;
use crate::vm::Result;

#[derive(Debug, Clone, Copy)]
pub enum Op<'a> {
    Operand(&'a Operand),
    Constant(&'a Constant),
}

impl<'a> From<&'a Operand> for Op<'a> {
    fn from(operand: &'a Operand) -> Self {
        Self::Operand(operand)
    }
}

impl<'a> From<&'a Constant> for Op<'a> {
    fn from(constant: &'a Constant) -> Self {
        Self::Constant(constant)
    }
}

impl<'a> From<&'a ConstantRef> for Op<'a> {
    fn from(constant: &'a ConstantRef) -> Self {
        Self::Constant(constant)
    }
}

impl<'a> Typed for Op<'a> {
    fn get_type(&self, types: &llvm_ir::types::Types) -> llvm_ir::TypeRef {
        match self {
            Op::Operand(operand) => operand.get_type(types),
            Op::Constant(constant) => constant.get_type(types),
        }
    }
}

impl<'a> ToValue<u64> for Op<'a> {
    fn to_value(&self) -> Result<u64> {
        match self {
            Op::Operand(operand) => operand.to_value(),
            Op::Constant(constant) => constant.to_value(),
        }
    }
}

impl<'a> Op<'a> {
    pub fn is_constant(&self) -> bool {
        match self {
            Op::Operand(op) => match op {
                Operand::LocalOperand { .. } => false,
                Operand::ConstantOperand(_) => true,
                Operand::MetadataOperand => false,
            },
            Op::Constant(_) => true,
        }
    }
}

use llvm_ir::types::Typed;
use llvm_ir::{Constant, ConstantRef, Operand};

use super::Size;
use crate::llvm::AsConcrete;
use crate::solver::BV;
use crate::vm::{Result, State, VMError};
use llvm_ir::Type;

#[derive(Clone, Copy)]
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

impl<'a> AsConcrete<u64> for Op<'a> {
    fn as_concrete(&self) -> anyhow::Result<u64, crate::vm::VMError> {
        match self {
            Op::Operand(operand) => operand.as_concrete(),
            Op::Constant(constant) => constant.as_concrete(),
        }
    }
}

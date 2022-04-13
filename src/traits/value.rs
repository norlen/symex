use llvm_ir::{Constant, ConstantRef, Operand};

use super::ToValue;
use crate::vm::Result;

impl ToValue<u64> for Operand {
    fn to_value(&self) -> Result<u64> {
        match self {
            Operand::LocalOperand { .. } => todo!(),
            Operand::ConstantOperand(constant) => constant.to_value(),
            Operand::MetadataOperand => todo!(),
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
        constant_to_u64(self)
    }
}

/// Returns the constant's u64 value if it can.
pub(super) fn constant_to_u64(constant: &Constant) -> Result<u64> {
    match constant {
        Constant::Int { value, .. } => Ok(*value),
        Constant::Float(_) => todo!(),
        Constant::Null(_) => todo!(),
        Constant::AggregateZero(_) => todo!(),
        Constant::Struct { .. } => todo!(),
        Constant::Array { .. } => todo!(),
        Constant::Vector(_) => todo!(),
        Constant::Undef(_) => todo!(),
        Constant::Poison(_) => todo!(),
        Constant::BlockAddress => todo!(),
        Constant::GlobalReference { .. } => todo!(),
        Constant::TokenNone => todo!(),
        Constant::Add(op) => {
            let lhs = constant_to_u64(&op.operand0)?;
            let rhs = constant_to_u64(&op.operand1)?;
            Ok(lhs + rhs)
        }
        Constant::Sub(op) => {
            let lhs = constant_to_u64(&op.operand0)?;
            let rhs = constant_to_u64(&op.operand1)?;
            Ok(lhs - rhs)
        }
        Constant::Mul(op) => {
            let lhs = constant_to_u64(&op.operand0)?;
            let rhs = constant_to_u64(&op.operand1)?;
            Ok(lhs * rhs)
        }
        Constant::UDiv(_) => todo!(),
        Constant::SDiv(_) => todo!(),
        Constant::URem(_) => todo!(),
        Constant::SRem(_) => todo!(),
        Constant::And(_) => todo!(),
        Constant::Or(_) => todo!(),
        Constant::Xor(_) => todo!(),
        Constant::Shl(_) => todo!(),
        Constant::LShr(_) => todo!(),
        Constant::AShr(_) => todo!(),
        Constant::FAdd(_) => todo!(),
        Constant::FSub(_) => todo!(),
        Constant::FMul(_) => todo!(),
        Constant::FDiv(_) => todo!(),
        Constant::FRem(_) => todo!(),
        Constant::ExtractElement(_) => todo!(),
        Constant::InsertElement(_) => todo!(),
        Constant::ShuffleVector(_) => todo!(),
        Constant::ExtractValue(_) => todo!(),
        Constant::InsertValue(_) => todo!(),
        Constant::GetElementPtr(_) => todo!(),
        Constant::Trunc(_) => todo!(),
        Constant::ZExt(_) => todo!(),
        Constant::SExt(_) => todo!(),
        Constant::FPTrunc(_) => todo!(),
        Constant::FPExt(_) => todo!(),
        Constant::FPToUI(_) => todo!(),
        Constant::FPToSI(_) => todo!(),
        Constant::UIToFP(_) => todo!(),
        Constant::SIToFP(_) => todo!(),
        Constant::PtrToInt(_) => todo!(),
        Constant::IntToPtr(_) => todo!(),
        Constant::BitCast(_) => todo!(),
        Constant::AddrSpaceCast(_) => todo!(),
        Constant::ICmp(_) => todo!(),
        Constant::FCmp(_) => todo!(),
        Constant::Select(_) => todo!(),
    }
}

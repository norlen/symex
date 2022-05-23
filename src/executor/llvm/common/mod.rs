use llvm_ir::{types::Typed, Constant, ConstantRef, Operand};

use super::Result;

mod ops;
mod size;
mod to_expr;

pub(crate) use ops::*;
pub use size::*;
pub use to_expr::*;

/// Op is a helper for values that can be either an [Operand] or a [Constant].
///
/// Most instructions operate on [Operand]s, but a [Constant] can contain these same instructions.
/// E.g. a [Constant] can contain the `getelementptr` functionality for calculating the offset
/// inside a constant. This Enum allows for creating functions that can be shared for both cases.
#[derive(Debug, Clone, Copy)]
pub enum Op<'a> {
    /// Contains a reference to an [Operand].
    Operand(&'a Operand),

    /// Contains a reference to a [Constant].
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

impl<'a> Op<'a> {
    /// Returns `true` if the [Op] is a [Constant], otherwise `false`.
    pub fn is_constant(&self) -> bool {
        match self {
            Op::Operand(op) => match op {
                Operand::ConstantOperand(_) => true,
                Operand::LocalOperand { .. } => false,
                Operand::MetadataOperand => false,
            },
            Op::Constant(_) => true,
        }
    }
}

/// Trait to get a value from [llvm_ir::Operand] or [llvm_ir::Constant].
///
/// The conversions are simple and retrieves an underlying value, they do not try to handle more
/// advanced expressions.
pub trait ToValue<T> {
    /// Tries to get the concrete value.
    fn to_value(&self) -> Result<T>;
}

// impl ToValue<u64> for Operand {
//     fn to_value(&self) -> Result<u64> {
//         match self {
//             Operand::LocalOperand { .. } => panic!("Cannot get constant from operand"),
//             Operand::ConstantOperand(constant) => constant.to_value(),
//             Operand::MetadataOperand => Err(LLVMExecutorError::MalformedInstruction),
//         }
//     }
// }

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

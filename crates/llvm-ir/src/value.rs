use llvm_sys::{
    core::{LLVMGetValueKind, LLVMIsConstant},
    prelude::LLVMValueRef,
    LLVMValueKind,
};

use crate::{constant::Constant, instruction::Instruction, Argument, Function, Global, Type};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Function(Function),
    Global(Global),
    Instruction(Instruction),
    Argument(Argument),
    Constant(Constant),
    Metadata,
    InlineAsm,
}

impl Value {
    pub(crate) fn new(value_ref: LLVMValueRef) -> Self {
        let value_kind = unsafe { LLVMGetValueKind(value_ref) };

        match value_kind {
            LLVMValueKind::LLVMArgumentValueKind => Self::Argument(Argument::new(value_ref)),
            LLVMValueKind::LLVMFunctionValueKind => Self::Function(Function::new(value_ref)),

            LLVMValueKind::LLVMGlobalAliasValueKind
            | LLVMValueKind::LLVMGlobalIFuncValueKind
            | LLVMValueKind::LLVMGlobalVariableValueKind => Self::Global(Global::new(value_ref)),

            LLVMValueKind::LLVMInstructionValueKind => {
                Self::Instruction(Instruction::new(value_ref))
            }

            LLVMValueKind::LLVMMetadataAsValueValueKind => Self::Metadata,

            LLVMValueKind::LLVMInlineAsmValueKind => Self::InlineAsm,

            // Try and treat the rest as constants.
            _ if unsafe { LLVMIsConstant(value_ref) != 0 } => {
                Self::Constant(Constant::new(value_ref))
            }

            _ => panic!(
                "Cannot use value kind {:?} as a top-level value",
                value_kind
            ),
        }
    }

    pub fn ty(&self) -> Type {
        match self {
            Value::Function(v) => v.ty(),
            Value::Global(v) => v.ty(),
            Value::Instruction(v) => v.result_type(),
            Value::Argument(v) => v.ty(),
            Value::Constant(v) => v.ty(),
            Value::Metadata => Type::Metadata,
            Value::InlineAsm => todo!("Inline assembly type"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Function(v) => write!(f, "{v}"),
            Value::Global(v) => write!(f, "{v}"),
            Value::Instruction(v) => write!(f, "{v}"),
            Value::Argument(v) => write!(f, "{v}"),
            Value::Constant(v) => write!(f, "{v}"),
            Value::Metadata => write!(f, "<metadata>"),
            Value::InlineAsm => write!(f, "<inline asm>"),
        }
    }
}

use std::error::Error;

use anyhow::anyhow;
use llvm_ir::constant::{self, GetElementPtr};
use llvm_ir::types::{FPType, NamedStructDef, Typed};
use llvm_ir::{Constant, ConstantRef, Operand, Type, TypeRef};
use log::trace;

use crate::memory::{to_bytes, BITS_IN_BYTE};
use crate::project::Project;
use crate::vm::{self, Result, State};
use crate::BV;

pub trait Size {
    fn size(&self, project: &Project) -> Option<u64>;

    fn size_in_bytes(&self, project: &Project) -> Result<Option<u64>>;

    fn offset(&self, index: u64, project: &Project) -> Option<(u64, TypeRef)>;

    fn offset_in_bytes(&self, index: u64, project: &Project) -> Result<Option<(u64, TypeRef)>>;

    fn inner_ty(&self, project: &Project) -> Option<TypeRef>;
}

impl Size for Type {
    fn size(&self, project: &Project) -> Option<u64> {
        size_in_bits(self, project)
    }

    fn offset(&self, index: u64, project: &Project) -> Option<(u64, TypeRef)> {
        get_offset(self, index, project)
    }

    fn inner_ty(&self, project: &Project) -> Option<TypeRef> {
        get_inner_type(self, project)
    }

    fn size_in_bytes(&self, project: &Project) -> Result<Option<u64>> {
        self.size(project).map(to_bytes).transpose()
    }

    fn offset_in_bytes(&self, index: u64, project: &Project) -> Result<Option<(u64, TypeRef)>> {
        match self.offset(index, project) {
            None => Ok(None),
            Some((size, ty)) => to_bytes(size).map(|s| Some((s, ty))),
        }
    }
}

impl Size for TypeRef {
    fn size(&self, project: &Project) -> Option<u64> {
        size_in_bits(self, project)
    }

    fn offset(&self, index: u64, project: &Project) -> Option<(u64, TypeRef)> {
        get_offset(self, index, project)
    }

    fn inner_ty(&self, project: &Project) -> Option<TypeRef> {
        get_inner_type(self, project)
    }

    fn size_in_bytes(&self, project: &Project) -> Result<Option<u64>> {
        self.size(project).map(to_bytes).transpose()
    }

    fn offset_in_bytes(&self, index: u64, project: &Project) -> Result<Option<(u64, TypeRef)>> {
        match self.offset(index, project) {
            None => Ok(None),
            Some((size, ty)) => to_bytes(size).map(|s| Some((s, ty))),
        }
    }
}

/// Calculates the size of the type in bits.
pub fn size_in_bits(ty: &Type, project: &Project) -> Option<u64> {
    use Type::*;

    match ty {
        VoidType => Some(0),
        IntegerType { bits } => Some(*bits as u64),
        PointerType { .. } => Some(project.ptr_size),
        FPType(fp_ty) => Some(fp_size_in_bits(fp_ty)),
        VectorType {
            element_type,
            num_elements,
            ..
        } => size_in_bits(element_type, project).map(|size| *num_elements as u64 * size),
        ArrayType {
            element_type,
            num_elements,
        } => size_in_bits(element_type, project).map(|size| *num_elements as u64 * size),
        StructType { element_types, .. } => element_types
            .iter()
            .map(|ty| size_in_bits(ty, project))
            .sum(),
        NamedStructType { name } => match project.get_named_struct(name)? {
            NamedStructDef::Opaque => None,
            NamedStructDef::Defined(ty) => size_in_bits(ty, project),
        },
        FuncType { .. } => todo!(),
        X86_MMXType => todo!(),
        X86_AMXType => todo!(),
        MetadataType => todo!(),
        LabelType => todo!(),
        TokenType => todo!(),
    }
}

/// Calculates an index based offset for a [`Type`].
pub fn get_offset(ty: &Type, index: u64, project: &Project) -> Option<(u64, TypeRef)> {
    use Type::*;

    match ty {
        PointerType {
            pointee_type,
            addr_space,
        } => Some((project.ptr_size * index, pointee_type.clone())),
        VectorType { element_type, .. } => {
            let size = size_in_bits(element_type, project)?;
            Some((size * index, element_type.clone()))
        }
        ArrayType {
            element_type,
            num_elements,
        } => {
            let size = size_in_bits(element_type, project)?;
            Some((size * index, element_type.clone()))
        }
        StructType { element_types, .. } => {
            let index = index as usize;

            let offset = element_types
                .iter()
                .take(index)
                .map(|ty| size_in_bits(ty, project))
                .sum::<Option<_>>()?;

            let inner_ty = element_types.get(index)?.clone();

            Some((offset, inner_ty))
        }
        NamedStructType { name } => match project.get_named_struct(name).unwrap() {
            NamedStructDef::Opaque => todo!(),
            NamedStructDef::Defined(ty) => get_offset(ty, index, project),
        },
        VoidType => todo!(),
        IntegerType { bits } => todo!(),
        FPType(_) => todo!(),
        FuncType {
            result_type,
            param_types,
            is_var_arg,
        } => todo!(),
        X86_MMXType => todo!(),
        X86_AMXType => todo!(),
        MetadataType => todo!(),
        LabelType => todo!(),
        TokenType => todo!(),
    }
}

/// Returns the inner type of the passed [`Type`].
///
/// If an inner type exists, it is returned in `Some`, otherwise `None` is
/// returned.
pub fn get_inner_type(ty: &Type, project: &Project) -> Option<TypeRef> {
    use Type::*;

    match ty {
        VoidType => None,
        IntegerType { .. } => None,
        PointerType { pointee_type, .. } => Some(pointee_type.clone()),
        FPType(_) => todo!(),
        FuncType { .. } => todo!(),
        VectorType { .. } => todo!(),
        ArrayType { element_type, .. } => Some(element_type.clone()),
        StructType { element_types, .. } => todo!(),
        NamedStructType { name } => todo!(),
        X86_MMXType => todo!(),
        X86_AMXType => todo!(),
        MetadataType => todo!(),
        LabelType => todo!(),
        TokenType => todo!(),
    }
}

/// Returns the size of a floating point type.
pub fn fp_size_in_bits(ty: &FPType) -> u64 {
    match ty {
        FPType::Half => 16,
        FPType::BFloat => 16,
        FPType::Single => 32,
        FPType::Double => 64,
        FPType::FP128 => 128,
        FPType::X86_FP80 => 80,
        FPType::PPC_FP128 => 128,
    }
}

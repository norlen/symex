use anyhow::anyhow;
use llvm_ir::types::{FPType, NamedStructDef, Type, TypeRef};

use super::Size;
use crate::{
    memory::to_bytes,
    project::Project,
    vm::{Result, State, VMError},
    BV,
};

impl Size for Type {
    fn size(&self, project: &Project) -> Option<u64> {
        size_in_bits(self, project)
    }

    fn size_in_bytes(&self, project: &Project) -> Result<Option<u64>> {
        self.size(project)
            .map(to_bytes)
            .transpose()
            .map_err(|err| VMError::MemoryError(err))
    }

    fn offset_constant(&self, index: u64, project: &Project) -> Result<(u64, TypeRef)> {
        get_offset_concrete(self, index, project)
    }

    fn offset_constant_in_bytes(&self, index: u64, project: &Project) -> Result<(u64, TypeRef)> {
        let (offset, ty) = self.offset_constant(index, project)?;
        let offset = to_bytes(offset).map_err(|err| VMError::MemoryError(err))?;
        Ok((offset, ty))
    }

    fn offset_symbol(&self, index: &BV, state: &mut State<'_>) -> Result<(BV, TypeRef)> {
        get_offset_bv(self, index, state)
    }

    fn inner_ty(&self, project: &Project) -> Option<TypeRef> {
        get_inner_type(self, project)
    }
}

impl Size for TypeRef {
    fn size(&self, project: &Project) -> Option<u64> {
        self.as_ref().size(project)
    }

    fn size_in_bytes(&self, project: &Project) -> Result<Option<u64>> {
        self.as_ref().size_in_bytes(project)
    }

    fn offset_constant(&self, index: u64, project: &Project) -> Result<(u64, TypeRef)> {
        self.as_ref().offset_constant(index, project)
    }

    fn offset_constant_in_bytes(&self, index: u64, project: &Project) -> Result<(u64, TypeRef)> {
        self.as_ref().offset_constant_in_bytes(index, project)
    }

    fn offset_symbol(&self, index: &BV, state: &mut State<'_>) -> Result<(BV, TypeRef)> {
        self.as_ref().offset_symbol(index, state)
    }

    fn inner_ty(&self, project: &Project) -> Option<TypeRef> {
        self.as_ref().inner_ty(project)
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
        } => size_in_bits(element_type, project).map(|size| {
            println!(
                "ARRAY TYPE: element_size: {}, num_elements: {}, total: {}",
                size,
                *num_elements,
                size * *num_elements as u64
            );
            *num_elements as u64 * size
        }),
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

pub fn get_offset_bv(ty: &Type, index: &BV, state: &mut State<'_>) -> Result<(BV, TypeRef)> {
    use Type::*;

    match ty {
        PointerType {
            pointee_type: inner_ty,
            ..
        }
        // LLVM docs:
        // Q: Can GEP index into vector elements?
        // A: This hasn’t always been forcefully disallowed, though it’s not
        //    recommended. It leads to awkward special cases in the optimizers, 
        //    and fundamental inconsistency in the IR. In the future, it will
        //    probably be outright disallowed.
        | VectorType {
            element_type: inner_ty,
            ..
        }
        | ArrayType {
            element_type: inner_ty,
            ..
        } => {
            let size = inner_ty.size(&state.project).unwrap();
            let size = state.solver.bv_from_u64(size, index.len());
            Ok((size.mul(index), inner_ty.clone()))
        }

        // Not supported for non-constant indexes.
        NamedStructType { .. } => todo!(),
        StructType { .. } => todo!(),

        // Should not happend for these instructions.
        VoidType => todo!(),
        IntegerType { .. } => todo!(),
        FPType(_) => todo!(),
        FuncType { .. } => todo!(),
        X86_MMXType => todo!(),
        X86_AMXType => todo!(),
        MetadataType => todo!(),
        LabelType => todo!(),
        TokenType => todo!(),
    }
}

/// Calculate the offset from a concrete index.
///
/// TODO: LLVM docs note that it is allowed for negative indices.
pub fn get_offset_concrete(ty: &Type, index: u64, project: &Project) -> Result<(u64, TypeRef)> {
    use Type::*;

    match ty {
        PointerType {
            pointee_type: inner_ty,
            ..
        }
        | VectorType {
            element_type: inner_ty,
            ..
        }
        | ArrayType {
            element_type: inner_ty,
            ..
        } => {
            let size = inner_ty.size(project).unwrap();
            Ok((size * index, inner_ty.clone()))
        }
        // TODO: Structs have an `is_packed`, this should probably affect the results...
        StructType { element_types, .. } => {
            let offset = element_types
                .iter()
                .take(index as usize)
                .map(|ty| size_in_bits(ty, project).ok_or_else(|| VMError::Other(anyhow!("Oops"))))
                .sum::<Result<u64>>()?;

            let ty = element_types.get(index as usize).cloned().unwrap();

            Ok((offset, ty))
        }
        NamedStructType { name } => match project.get_named_struct(name).unwrap() {
            NamedStructDef::Opaque => todo!(),
            NamedStructDef::Defined(ty) => get_offset_concrete(ty, index, project),
        },

        // Should not happend for these instructions.
        VoidType => todo!(),
        IntegerType { .. } => todo!(),
        FPType(_) => todo!(),
        FuncType { .. } => todo!(),
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
        StructType { .. } => todo!(),
        NamedStructType { name } => match project.get_named_struct(name).unwrap() {
            NamedStructDef::Opaque => todo!(),
            NamedStructDef::Defined(ty) => get_inner_type(ty, project),
        },
        X86_MMXType => todo!(),
        X86_AMXType => todo!(),
        MetadataType => todo!(),
        LabelType => todo!(),
        TokenType => todo!(),
    }
}

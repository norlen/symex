#![allow(unused_variables)]

use anyhow::{anyhow, Result};
use llvm_ir::constant;
use llvm_ir::types::{FPType, NamedStructDef};
use llvm_ir::{Constant, Operand, Type, TypeRef};
use log::trace;

use crate::project::Project;
use crate::vm::State;
use crate::BV;

// pub enum Call<'a> {
//     Call(&'a instruction::Call),
//     Invoke(&'a Invoke),
// }

pub fn u64_from_operand(op: &Operand) -> Result<u64> {
    match op {
        Operand::LocalOperand { .. } => todo!(),
        Operand::ConstantOperand(constant) => u64_from_constant(constant),
        Operand::MetadataOperand => todo!(),
    }
}

pub fn u64_from_constant(constant: &Constant) -> Result<u64> {
    match constant {
        Constant::Int { value, .. } => Ok(*value),
        Constant::Float(_) => todo!(),
        Constant::Null(_) => todo!(),
        Constant::AggregateZero(_) => todo!(),
        Constant::Struct {
            name,
            values,
            is_packed,
        } => todo!(),
        Constant::Array {
            element_type,
            elements,
        } => todo!(),
        Constant::Vector(_) => todo!(),
        Constant::Undef(_) => todo!(),
        Constant::Poison(_) => todo!(),
        Constant::BlockAddress => todo!(),
        Constant::GlobalReference { name, ty } => todo!(),
        Constant::TokenNone => todo!(),
        Constant::Add(constant::Add { operand0, operand1 }) => {
            let lhs = u64_from_constant(operand0)?;
            let rhs = u64_from_constant(operand1)?;
            Ok(lhs + rhs)
        }
        Constant::Sub(constant::Sub { operand0, operand1 }) => {
            let lhs = u64_from_constant(operand0)?;
            let rhs = u64_from_constant(operand1)?;
            Ok(lhs - rhs)
        }
        Constant::Mul(_) => todo!(),
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

// pub fn get_constant_int_value(op: &Operand) -> Result<usize> {
//     match op {
//         Operand::ConstantOperand(constant_ref) => match constant_ref.as_ref() {
//             Constant::Int { value, .. } => Ok(*value as usize),
//             constant => Err(anyhow!(
//                 "alloca expected NumElements to be constant int, got {:?}",
//                 constant
//             )),
//         },
//         operand => Err(anyhow!(
//             "alloca expected NumElements to be constant int, got {:?}",
//             operand
//         )),
//     }
// }

pub fn operand_to_bv(op: &Operand, state: &mut State<'_>) -> Result<BV> {
    match op {
        Operand::ConstantOperand(c) => const_to_bv(c, state),
        Operand::LocalOperand { name, .. } => {
            trace!("operand_to_bv for name: {:?}", name);
            let v = state.vars.get(name).unwrap().clone();
            Ok(v)
        }
        Operand::MetadataOperand => Err(anyhow!("Cannot convert {:?} to BV", op)),
    }
}

pub fn const_to_bv(constant: &Constant, state: &mut State<'_>) -> Result<BV> {
    match constant {
        Constant::Int { bits, value } => Ok(state.solver.bv_from_u64(*value, *bits)),
        Constant::Float(_) => todo!(),
        Constant::Null(_) => todo!(),
        Constant::AggregateZero(_) => todo!(),
        Constant::Struct {
            name,
            values,
            is_packed,
        } => todo!(),
        Constant::Array {
            element_type,
            elements,
        } => todo!(),
        Constant::Vector(_) => todo!(),
        Constant::Undef(_) => todo!(),
        Constant::Poison(_) => todo!(),
        Constant::BlockAddress => todo!(),
        Constant::GlobalReference { name, ty } => todo!(),
        Constant::TokenNone => todo!(),
        Constant::Add(op) => {
            let op0 = const_to_bv(&op.operand0, state)?;
            let op1 = const_to_bv(&op.operand1, state)?;
            Ok(op0.add(&op1))
        }
        Constant::Sub(op) => {
            let op0 = const_to_bv(&op.operand0, state)?;
            let op1 = const_to_bv(&op.operand1, state)?;
            Ok(op0.sub(&op1))
        }
        Constant::Mul(_) => todo!(),
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

pub fn get_offset_in_bits(ty: &TypeRef, index: usize, project: &Project) -> (usize, TypeRef) {
    match ty.as_ref() {
        Type::VoidType => todo!(),
        Type::IntegerType { bits } => todo!(),
        Type::PointerType {
            pointee_type,
            addr_space,
        } => (project.ptr_size * index, pointee_type.clone()),
        Type::FPType(_) => todo!(),
        Type::FuncType {
            result_type,
            param_types,
            is_var_arg,
        } => todo!(),
        Type::VectorType { element_type, .. } => {
            let size = size_in_bits(element_type, project).unwrap();
            (size * index, element_type.clone())
        }
        Type::ArrayType {
            element_type,
            num_elements,
        } => {
            let size = size_in_bits(element_type, project).unwrap();
            (size * index, element_type.clone())
        }
        Type::StructType { element_types, .. } => {
            let mut offset = 0;
            for ty in element_types.iter().take(index) {
                offset += size_in_bits(ty, project).unwrap();
            }
            (offset, element_types.get(index).unwrap().clone())
        }
        Type::NamedStructType { name } => match project.get_named_struct(name).unwrap() {
            NamedStructDef::Opaque => todo!(),
            NamedStructDef::Defined(ty) => get_offset_in_bits(ty, index, project),
        },
        Type::X86_MMXType => todo!(),
        Type::X86_AMXType => todo!(),
        Type::MetadataType => todo!(),
        Type::LabelType => todo!(),
        Type::TokenType => todo!(),
    }
}

pub fn get_inner_type(ty: &TypeRef, project: &Project) -> Option<TypeRef> {
    match ty.as_ref() {
        Type::VoidType => None,
        Type::IntegerType { .. } => None,
        Type::PointerType { pointee_type, .. } => Some(pointee_type.clone()),
        Type::FPType(_) => todo!(),
        Type::FuncType { .. } => todo!(),
        Type::VectorType { .. } => todo!(),
        Type::ArrayType { element_type, .. } => Some(element_type.clone()),
        Type::StructType { element_types, .. } => todo!(),
        Type::NamedStructType { name } => todo!(),
        Type::X86_MMXType => todo!(),
        Type::X86_AMXType => todo!(),
        Type::MetadataType => todo!(),
        Type::LabelType => todo!(),
        Type::TokenType => todo!(),
    }
}

pub fn size_in_bits(ty: &Type, project: &Project) -> Option<usize> {
    match ty {
        Type::VoidType => Some(0),
        Type::IntegerType { bits } => Some(*bits as usize),
        Type::PointerType { .. } => Some(project.ptr_size),
        Type::FPType(fp_ty) => Some(fp_size_in_bits(fp_ty)),
        Type::FuncType { .. } => todo!(),
        Type::VectorType {
            element_type,
            num_elements,
            ..
        } => {
            let element_size = size_in_bits(element_type, project);
            element_size.map(|size| size * *num_elements)
        }
        Type::ArrayType {
            element_type,
            num_elements,
        } => {
            let element_size = size_in_bits(element_type, project);
            element_size.map(|size| size * *num_elements)
        }
        Type::StructType { element_types, .. } => element_types
            .iter()
            .map(|ty| size_in_bits(ty, project))
            .sum(),
        Type::NamedStructType { name } => match project.get_named_struct(name)? {
            NamedStructDef::Opaque => None,
            NamedStructDef::Defined(ty) => size_in_bits(ty, project),
        },
        Type::X86_MMXType => todo!(),
        Type::X86_AMXType => todo!(),
        Type::MetadataType => todo!(),
        Type::LabelType => todo!(),
        Type::TokenType => todo!(),
    }
}

pub fn size_in_bytes(ty: &Type, project: &Project) -> Result<Option<usize>> {
    match size_in_bits(ty, project) {
        Some(size) => {
            const BITS_IN_BYTES: usize = 8;
            if size % 8 != 0 {
                Err(anyhow!("size not a multiple of 8"))
            } else {
                Ok(Some(size / BITS_IN_BYTES))
            }
        }
        None => Ok(None),
    }
}

fn fp_size_in_bits(ty: &FPType) -> usize {
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

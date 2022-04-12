#![allow(unused_variables)]

use std::error::Error;
use std::fs::OpenOptions;

use anyhow::anyhow;
use llvm_ir::constant::{self, GetElementPtr};
use llvm_ir::types::{FPType, NamedStructDef, Typed};
use llvm_ir::{Constant, ConstantRef, Operand, Type, TypeRef};
use log::{trace, warn};

use crate::llvm::Size;
use crate::memory::BITS_IN_BYTE;
use crate::project::Project;
use crate::solver::BinaryOperation;
use crate::util::gep_constant;
use crate::vm::ops::binop_const;
use crate::vm::{Result, State};
use crate::BV;

/// Converts an [`Operand`] to a [`BV`].
pub fn operand_to_bv(op: &Operand, state: &mut State<'_>) -> Result<BV> {
    // println!("op: {:?}", op);
    match op {
        Operand::ConstantOperand(c) => const_to_bv(c, state),
        Operand::LocalOperand { name, .. } => {
            trace!("operand_to_bv for name: {:?}", name);
            let v = state.vars.get(name).unwrap().clone();
            Ok(v)
        }
        Operand::MetadataOperand => Err(anyhow!("Cannot convert {:?} to BV", op).into()),
    }
}

/// Converts a [`Constant`] to a [`BV`].
pub fn const_to_bv(constant: &Constant, state: &mut State<'_>) -> Result<BV> {
    match constant {
        Constant::Int { bits, value } => Ok(state.solver.bv_from_u64(*value, *bits)),
        Constant::Float(_) => todo!(),
        Constant::Null(ty) | Constant::AggregateZero(ty) => {
            let size = ty.size(&state.project).unwrap();
            let size = if size == 0 {
                warn!("zero width const to bv");
                4
            } else {
                size
            };
            Ok(state.solver.bv_zero(size as u32))
        }
        Constant::Struct {
            values: elements, ..
        }
        | Constant::Array { elements, .. }
        | Constant::Vector(elements) => {
            let bvs = elements.iter().map(|c| const_to_bv(c, state));

            let combined = bvs.reduce(|acc, v| {
                let v = v?;
                let acc = acc?;
                Ok(v.concat(&acc).into())
            });
            combined.unwrap()
        }
        Constant::Undef(_) => todo!(),
        Constant::Poison(_) => todo!(),
        Constant::BlockAddress => todo!(),
        Constant::GlobalReference { name, ty } => {
            if let Some(global) = state.get_global(name).cloned() {
                match &global.kind {
                    crate::vm::AllocationType::Variable(v) => {
                        if !v.initialized.get() {
                            let value = state.get_bv_from_constant(&v.initializer)?;
                            state.mem.write(&global.addr_bv, value)?;
                            v.initialized.set(true);
                        }
                    }
                    crate::vm::AllocationType::Function(_) => {}
                }
                Ok(global.addr_bv.clone())
            } else {
                panic!("global ref not found");
            }
        }
        Constant::TokenNone => todo!(),
        Constant::Add(op) => binop_const(state, &op.operand0, &op.operand1, BinaryOperation::Add),
        Constant::Sub(op) => binop_const(state, &op.operand0, &op.operand1, BinaryOperation::Sub),
        Constant::Mul(op) => binop_const(state, &op.operand0, &op.operand1, BinaryOperation::Mul),
        Constant::UDiv(op) => binop_const(state, &op.operand0, &op.operand1, BinaryOperation::UDiv),
        Constant::SDiv(op) => binop_const(state, &op.operand0, &op.operand1, BinaryOperation::SDiv),
        Constant::URem(op) => binop_const(state, &op.operand0, &op.operand1, BinaryOperation::URem),
        Constant::SRem(op) => binop_const(state, &op.operand0, &op.operand1, BinaryOperation::SRem),
        Constant::And(op) => binop_const(state, &op.operand0, &op.operand1, BinaryOperation::And),
        Constant::Or(op) => binop_const(state, &op.operand0, &op.operand1, BinaryOperation::Or),
        Constant::Xor(op) => binop_const(state, &op.operand0, &op.operand1, BinaryOperation::Xor),
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
        Constant::GetElementPtr(GetElementPtr {
            address,
            indices,
            in_bounds,
        }) => {
            let target_address = gep_constant(address, indices, *in_bounds, state).unwrap();
            Ok(target_address)
        }
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
        Constant::BitCast(op) => {
            let value = const_to_bv(&op.operand, state)?;
            assert_eq!(value.len() as u64, op.to_type.size(state.project).unwrap());
            Ok(value)
        }
        Constant::AddrSpaceCast(_) => todo!(),
        Constant::ICmp(_) => todo!(),
        Constant::FCmp(_) => todo!(),
        Constant::Select(_) => todo!(),
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

/// Returns the constant's u64 value if it can.
pub fn constant_as_u64(constant: &Constant) -> Result<u64> {
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
            let lhs = constant_as_u64(operand0)?;
            let rhs = constant_as_u64(operand1)?;
            Ok(lhs + rhs)
        }
        Constant::Sub(constant::Sub { operand0, operand1 }) => {
            let lhs = constant_as_u64(operand0)?;
            let rhs = constant_as_u64(operand1)?;
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

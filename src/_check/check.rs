pub fn get_offset_in_bits(ty: &TypeRef, index: u64, project: &Project) -> Option<(u64, TypeRef)> {
    use Type::*;

    match ty.as_ref() {
        PointerType {
            pointee_type,
            addr_space,
        } => Some((project.ptr_size * index, pointee_type.clone())),
        VectorType { element_type, .. } => {
            let size = size_in_bits(element_type, project).unwrap();
            Some((size * index, element_type.clone()))
        }
        ArrayType {
            element_type,
            num_elements,
        } => {
            let size = size_in_bits(element_type, project).unwrap();
            Some((size * index, element_type.clone()))
        }
        StructType { element_types, .. } => {
            let mut offset = 0;
            for ty in element_types.iter().take(index as usize) {
                offset += size_in_bits(ty, project).unwrap();
            }
            Some((offset, element_types.get(index as usize).unwrap().clone()))
        }
        NamedStructType { name } => match project.get_named_struct(name).unwrap() {
            NamedStructDef::Opaque => todo!(),
            NamedStructDef::Defined(ty) => get_offset_in_bits(ty, index, project),
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

pub fn size_in_bits(ty: &Type, project: &Project) -> Option<u32> {
    match ty {
        Type::VoidType => Some(0),
        Type::IntegerType { bits } => Some(*bits),
        Type::PointerType { .. } => Some(project.ptr_size),
        Type::FPType(fp_ty) => Some(fp_size_in_bits(fp_ty)),
        Type::FuncType { .. } => todo!(),
        Type::VectorType {
            element_type,
            num_elements,
            ..
        } => {
            let element_size = size_in_bits(element_type, project);
            element_size.map(|size| size * *num_elements as u32)
        }
        Type::ArrayType {
            element_type,
            num_elements,
        } => {
            let element_size = size_in_bits(element_type, project);
            element_size.map(|size| size * *num_elements as u32)
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

pub fn size_in_bytes(ty: &Type, project: &Project) -> Result<Option<u32>> {
    match size_in_bits(ty, project) {
        Some(size) => {
            if size % 8 != 0 {
                Err(anyhow!("size not a multiple of 8"))
            } else {
                Ok(Some(size / BITS_IN_BYTE))
            }
        }
        None => Ok(None),
    }
}

fn fp_size_in_bits(ty: &FPType) -> u32 {
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

// pub fn u64_from_operand(op: &Operand) -> Result<u64> {
//     match op {
//         Operand::LocalOperand { .. } => todo!(),
//         Operand::ConstantOperand(constant) => u64_from_constant(constant),
//         Operand::MetadataOperand => todo!(),
//     }
// }

// pub fn u64_from_constant(constant: &Constant) -> Result<u64> {
//     match constant {
//         Constant::Int { value, .. } => Ok(*value),
//         Constant::Float(_) => todo!(),
//         Constant::Null(_) => todo!(),
//         Constant::AggregateZero(_) => todo!(),
//         Constant::Struct {
//             name,
//             values,
//             is_packed,
//         } => todo!(),
//         Constant::Array {
//             element_type,
//             elements,
//         } => todo!(),
//         Constant::Vector(_) => todo!(),
//         Constant::Undef(_) => todo!(),
//         Constant::Poison(_) => todo!(),
//         Constant::BlockAddress => todo!(),
//         Constant::GlobalReference { name, ty } => todo!(),
//         Constant::TokenNone => todo!(),
//         Constant::Add(constant::Add { operand0, operand1 }) => {
//             let lhs = u64_from_constant(operand0)?;
//             let rhs = u64_from_constant(operand1)?;
//             Ok(lhs + rhs)
//         }
//         Constant::Sub(constant::Sub { operand0, operand1 }) => {
//             let lhs = u64_from_constant(operand0)?;
//             let rhs = u64_from_constant(operand1)?;
//             Ok(lhs - rhs)
//         }
//         Constant::Mul(_) => todo!(),
//         Constant::UDiv(_) => todo!(),
//         Constant::SDiv(_) => todo!(),
//         Constant::URem(_) => todo!(),
//         Constant::SRem(_) => todo!(),
//         Constant::And(_) => todo!(),
//         Constant::Or(_) => todo!(),
//         Constant::Xor(_) => todo!(),
//         Constant::Shl(_) => todo!(),
//         Constant::LShr(_) => todo!(),
//         Constant::AShr(_) => todo!(),
//         Constant::FAdd(_) => todo!(),
//         Constant::FSub(_) => todo!(),
//         Constant::FMul(_) => todo!(),
//         Constant::FDiv(_) => todo!(),
//         Constant::FRem(_) => todo!(),
//         Constant::ExtractElement(_) => todo!(),
//         Constant::InsertElement(_) => todo!(),
//         Constant::ShuffleVector(_) => todo!(),
//         Constant::ExtractValue(_) => todo!(),
//         Constant::InsertValue(_) => todo!(),
//         Constant::GetElementPtr(_) => todo!(),
//         Constant::Trunc(_) => todo!(),
//         Constant::ZExt(_) => todo!(),
//         Constant::SExt(_) => todo!(),
//         Constant::FPTrunc(_) => todo!(),
//         Constant::FPExt(_) => todo!(),
//         Constant::FPToUI(_) => todo!(),
//         Constant::FPToSI(_) => todo!(),
//         Constant::UIToFP(_) => todo!(),
//         Constant::SIToFP(_) => todo!(),
//         Constant::PtrToInt(_) => todo!(),
//         Constant::IntToPtr(_) => todo!(),
//         Constant::BitCast(_) => todo!(),
//         Constant::AddrSpaceCast(_) => todo!(),
//         Constant::ICmp(_) => todo!(),
//         Constant::FCmp(_) => todo!(),
//         Constant::Select(_) => todo!(),
//     }
// }

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

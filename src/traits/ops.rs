use std::cmp::Ordering;

use llvm_ir::{IntPredicate, Operand, Type};

use super::{get_bit_offset_concrete, Op, ToValue};
use crate::{
    solver::{BinaryOperation, BV},
    traits::{get_byte_offset_concrete, get_byte_offset_symbol},
    vm::{Result, State, VMError},
};

pub(crate) fn extract_value<'p, T>(
    state: &mut State<'_>,
    aggregate: T,
    indices: &[u32],
) -> Result<BV>
where
    T: Into<Op<'p>>,
{
    let aggregate = aggregate.into();

    // Calculate the offset where the element is at.
    let mut ty = state.type_of(&aggregate);
    let mut total_offset = 0;

    for index in indices.iter().copied() {
        let (offset, inner_ty) = get_bit_offset_concrete(&ty, index as u64, &state.project)?;
        total_offset += offset;
        ty = inner_ty;
    }

    let offset_upper_bound = total_offset + state.project.bit_size(&ty)? as u64;

    // Get the value and check that the BV is big enough to accomodate our slice.
    let value = state.get_var(aggregate)?;
    assert!(value.len() >= offset_upper_bound as u32);

    let value = value.slice(total_offset as u32, offset_upper_bound as u32 - 1);
    Ok(value)
}

pub(crate) fn gep<'p, T, I>(
    state: &mut State<'_>,
    address: T,
    indices: I,
    _in_bounds: bool,
) -> Result<BV>
where
    T: Into<Op<'p>>,
    I: Iterator<Item = Op<'p>>,
{
    // The `in_bounds` field is pretty useless for figuring out if the address
    // is actually within the type. We cannot use any type information here
    // (https://llvm.org/docs/GetElementPtr.html)
    //
    // So we have to get the actual underlying allocation for this, but as the
    // address is symbolic that poses a problem.
    let address = address.into();
    let mut addr = state.get_var(address).unwrap();
    let ptr_size = addr.len();

    // The offsets modifies the address ptr, and this is the type of what
    // is currently pointed to.
    let mut curr_ty = state.type_of(&address);
    for index in indices {
        let (offset, ty) = if index.is_constant() {
            let index = index.to_value()?;
            let (offset, ty) = get_byte_offset_concrete(&curr_ty, index, &state.project)?;

            let offset = state.solver.bv_from_u64(offset, ptr_size);
            (offset, ty)
        } else {
            let index = state.get_var(index)?;
            get_byte_offset_symbol(&curr_ty, &index, &state.project)?
        };

        addr = addr.add(&offset);
        curr_ty = ty;
    }

    Ok(addr)
}

pub(crate) fn binop<'p, T>(state: &mut State<'_>, lhs: T, rhs: T, op: BinaryOperation) -> Result<BV>
where
    T: Into<Op<'p>>,
{
    // TODO: Could check that types match?
    // let lhs_ty = self.state.type_of(lhs);
    // let rhs_ty = self.state.type_of(rhs);

    let lhs = lhs.into();
    let rhs = rhs.into();
    assert_eq!(state.type_of(&lhs), state.type_of(&rhs));

    let lhs = state.get_var(lhs)?;
    let rhs = state.get_var(rhs)?;
    assert_eq!(lhs.len(), rhs.len());

    let result = match op {
        BinaryOperation::Add => lhs.add(&rhs),
        BinaryOperation::Sub => lhs.sub(&rhs),
        BinaryOperation::Mul => lhs.mul(&rhs),
        BinaryOperation::UDiv => lhs.udiv(&rhs),
        BinaryOperation::SDiv => lhs.sdiv(&rhs),
        BinaryOperation::URem => lhs.urem(&rhs),
        BinaryOperation::SRem => lhs.srem(&rhs),
        BinaryOperation::And => lhs.and(&rhs),
        BinaryOperation::Or => lhs.or(&rhs),
        BinaryOperation::Xor => lhs.xor(&rhs),
        BinaryOperation::Sll => lhs.sll(&rhs),
        BinaryOperation::Srl => lhs.srl(&rhs),
        BinaryOperation::Sra => lhs.sra(&rhs),
    };
    Ok(result)
}

/// Converts integers, pointers, or vectors.
///
/// Performs a conversion on either (int,int), (ptr,int), (int,ptr), or (vector,vector) with the
/// passed mapping function.
///
/// No type checking is done, if this is of interest they have to be checked before calling this
/// function.
pub(crate) fn convert_to_map<F>(
    state: &mut State<'_>,
    ty: &Type,
    op: &Operand,
    map: F,
) -> Result<BV>
where
    F: Fn(BV, u32) -> BV,
{
    let symbol = state.get_var(op)?;
    let source_ty = state.type_of(op);
    // For ptr->int or int->ptr the functionality is pretty simple. Just truncate, zero extend,
    // or cast depending on the target size.

    use Type::*;
    match (source_ty.as_ref(), ty) {
        // Integer to integer conversion are done by trunc, zext, and sext. While the ptr->int,
        // int->ptr are done by ptrtoint and inttoptr. All these should be supported.
        (IntegerType { .. }, IntegerType { .. })
        | (IntegerType { .. }, PointerType { .. })
        | (PointerType { .. }, IntegerType { .. }) => {
            let target_bits = state.project.bit_size(ty)?;
            Ok(map(symbol, target_bits))
        }

        // Vectors are a bit more annoying, in that the elements have to be processed one by one.
        (
            VectorType {
                element_type: source_element_ty,
                num_elements,
                scalable: false,
                ..
            },
            VectorType {
                element_type,
                scalable: false,
                ..
            },
        ) => {
            let source_bits = state.project.bit_size(source_element_ty)?;
            let target_bits = state.project.bit_size(element_type)?;
            let num_elements = *num_elements as u32;
            assert!(source_bits == symbol.len());

            // Process each element one by one and concatenate the result.
            (0..num_elements)
                .map(|i| {
                    let low = i * source_bits;
                    let high = (i + 1) * source_bits - 1;
                    let symbol = symbol.slice(low, high);
                    map(symbol, target_bits)
                })
                .reduce(|acc, v| v.concat(&acc))
                .ok_or_else(|| VMError::MalformedInstruction)
        }

        // TODO: Check if scalable vectors are similar
        (VectorType { .. }, VectorType { .. }) => Err(VMError::UnsupportedInstruction),

        // The other types should not appear for this instruction.
        _ => Err(VMError::MalformedInstruction),
    }
}

/// Convert operand to type `ty`.
///
/// Converting works for different bit widths, if the target is larger it is zero extended. If the
/// target is smaller the operand is truncated. Finally, if the are the same this is the same as
/// a cast.
pub(crate) fn convert_to(state: &mut State<'_>, ty: &Type, op: &Operand) -> Result<BV> {
    fn convert_symbol(symbol: BV, target_bits: u32) -> BV {
        match symbol.len().cmp(&target_bits) {
            Ordering::Equal => symbol,
            Ordering::Less => symbol.slice(0, target_bits - 1),
            Ordering::Greater => symbol.zero_ext(target_bits),
        }
    }
    convert_to_map(state, ty, op, convert_symbol)

    // let op = op.into();
    // let symbol = state.get_var(op)?;

    // let source_ty = state.type_of(&op);

    // use Type::*;
    // match (source_ty.as_ref(), ty) {
    //     // For ptr->int or int->ptr the functionality is pretty simple. Just truncate, zero extend,
    //     // or cast depending on the target size.
    //     (IntegerType { .. }, PointerType { .. }) | (PointerType { .. }, IntegerType { .. }) => {
    //         let target_bits = state.project.bit_size(ty)?;
    //         Ok(convert_symbol(symbol, target_bits))
    //     }

    //     // For vectors they have to be processed one by one, and the elements may be truncatated
    //     // or zero extended.
    //     (
    //         VectorType {
    //             element_type: source_element_ty,
    //             num_elements,
    //             scalable: false,
    //             ..
    //         },
    //         VectorType {
    //             element_type,
    //             scalable: false,
    //             ..
    //         },
    //     ) => {
    //         let source_bits = state.project.bit_size(source_element_ty)?;
    //         let target_bits = state.project.bit_size(element_type)?;
    //         let num_elements = *num_elements as u32;

    //         // Process each element one by one and concatenate the result.
    //         (0..num_elements)
    //             .map(|i| {
    //                 let low = i * source_bits;
    //                 let high = (i + 1) * source_bits - 1;
    //                 convert_symbol(symbol.slice(low, high), target_bits)
    //             })
    //             .reduce(|acc, v| v.concat(&acc))
    //             .ok_or_else(|| VMError::MalformedInstruction)
    //     }

    //     // TODO: Check if scalable vectors are similar
    //     (VectorType { .. }, VectorType { .. }) => Err(VMError::UnsupportedInstruction),

    //     // The other types should not appear for this instruction.
    //     _ => Err(VMError::MalformedInstruction),
    // }
}

/// Cast operand to type `ty`.
///
/// Casting simply reinterprets the bits as a different type. As the system does not return types,
/// this just returns the underlying symbol. The bitwidths must match.
pub(crate) fn cast_to<'p, T>(state: &mut State<'_>, ty: &Type, op: T) -> Result<BV>
where
    T: Into<Op<'p>>,
{
    let bv = state.get_var(op.into())?;
    assert_eq!(bv.len(), state.project.bit_size(ty).unwrap() as u32);
    Ok(bv)
}

pub(crate) fn icmp<'p, T>(
    state: &mut State<'_>,
    lhs: T,
    rhs: T,
    predicate: IntPredicate,
) -> Result<BV>
where
    T: Into<Op<'p>>,
{
    let lhs = state.get_var(lhs.into())?;
    let rhs = state.get_var(rhs.into())?;
    let result = match predicate {
        IntPredicate::EQ => lhs.eq(&rhs),
        IntPredicate::NE => lhs.ne(&rhs),
        IntPredicate::UGT => lhs.ugt(&rhs),
        IntPredicate::UGE => lhs.ugte(&rhs),
        IntPredicate::ULT => lhs.ult(&rhs),
        IntPredicate::ULE => lhs.ulte(&rhs),
        IntPredicate::SGT => lhs.sgt(&rhs),
        IntPredicate::SGE => lhs.sgte(&rhs),
        IntPredicate::SLT => lhs.slt(&rhs),
        IntPredicate::SLE => lhs.slte(&rhs),
    };
    Ok(result)
}

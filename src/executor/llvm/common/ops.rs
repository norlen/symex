use llvm_ir::{IntPredicate, Operand, Type};
use tracing::error;

use super::get_bit_offset_concrete;
use super::Op;
use crate::{
    executor::llvm::{LLVMExecutorError, LLVMState, Result},
    smt::{DExpr, Expression},
};

/// Calculate start and end offset into the aggregate.
pub(crate) fn get_element_offset(
    state: &LLVMState,
    aggregate: &Operand,
    indices: &[u32],
) -> Result<(u64, u64)> {
    let mut ty = state.type_of(aggregate);
    let mut total_offset = 0;
    for index in indices.iter().copied() {
        let (offset, inner_ty) = get_bit_offset_concrete(&ty, index as u64, &state.project)?;

        total_offset += offset;
        ty = inner_ty;
    }

    let element_size = state.project.bit_size(&ty)?;
    let upper_bound = total_offset + element_size as u64;

    Ok((total_offset, upper_bound))
}

pub(crate) fn extract_value(
    state: &LLVMState,
    aggregate: &Operand,
    indices: &[u32],
) -> Result<DExpr> {
    let (total_offset, offset_upper_bound) = get_element_offset(state, aggregate, indices)?;

    // Get the value and check that the BV is big enough to accommodate our slice.
    let value = state.get_expr(aggregate)?;
    assert!(value.len() >= offset_upper_bound as u32);

    let value = value.slice(total_offset as u32, offset_upper_bound as u32 - 1);
    Ok(value)
}

pub(crate) fn gep<'p, T, I>(
    state: &LLVMState,
    address: T,
    indices: I,
    _in_bounds: bool,
) -> Result<DExpr>
where
    T: Into<Op<'p>>,
    I: Iterator<Item = Op<'p>>,
{
    // The `in_bounds` field is pretty useless for figuring out if the address is actually within
    // the type. We cannot use any type information here (https://llvm.org/docs/GetElementPtr.html)
    //
    // So we have to get the actual underlying allocation for this, but as the address is symbolic
    // that poses a problem.
    let address = address.into();
    let mut addr = state.get_expr(address).unwrap();
    let ptr_size = addr.len();

    // The offsets modifies the address ptr, and this is the type of what is currently pointed to.
    let mut curr_ty = state.type_of(&address);
    if !matches!(curr_ty.as_ref(), Type::PointerType { .. }) {
        error!("getelementptr address should always be a pointer");
        return Err(LLVMExecutorError::MalformedInstruction);
    }

    for index in indices {
        let index = state
            .get_expr(index)?
            .zero_ext(state.project.ptr_size)
            .simplify();
        println!("index: {index:?}");

        let (offset, ty) = state.project.byte_offset(&curr_ty, &index, &state.ctx)?;

        addr = addr.add(&offset);
        curr_ty = ty;
    }

    Ok(addr)
}

/// Perform a binary operation on two operands, returning the result.
///
/// The input types must be either integers or a vector of integers. Vector operations are performed
/// on a per element basis.
///
/// TODO: No operations currently care about overflows and such.
pub(crate) fn binop<F>(
    state: &LLVMState,
    lhs: &Operand,
    rhs: &Operand,
    operation: F,
) -> Result<DExpr>
where
    F: Fn(&DExpr, &DExpr) -> DExpr,
{
    let lhs_ty = state.type_of(lhs);
    let rhs_ty = state.type_of(rhs);
    let lhs = state.get_expr(lhs)?;
    let rhs = state.get_expr(rhs)?;

    use Type::*;
    match (lhs_ty.as_ref(), rhs_ty.as_ref()) {
        // For simple integer types the result is trivial to do, just perform the operation.
        (IntegerType { .. }, IntegerType { .. }) => Ok(operation(&lhs, &rhs)),

        // Some may support pointer operands, such as icmp, which work the same as integer ones.
        (PointerType { .. }, PointerType { .. }) => Ok(operation(&lhs, &rhs)),

        // The docs do not really specify how the vector operations should work. But I'll assume it
        // is the operation on a per element basis.
        (
            VectorType {
                element_type: lhs_inner_ty,
                num_elements: n,
                scalable: false,
            },
            VectorType {
                element_type: rhs_inner_ty,
                num_elements: m,
                scalable: false,
            },
        ) => {
            assert_eq!(lhs_inner_ty, rhs_inner_ty);
            assert_eq!(
                state.project.bit_size(lhs_inner_ty)?,
                state.project.bit_size(rhs_inner_ty)?
            );
            assert_eq!(*n, *m);

            let bits = state.project.bit_size(lhs_inner_ty)?;
            let num_elements = *n as u32;

            // Perform the operation per element and concatenate the result.
            (0..num_elements)
                .map(|i| {
                    let low = i * bits;
                    let high = (i + 1) * bits - 1;
                    let lhs = lhs.slice(low, high);
                    let rhs = rhs.slice(low, high);
                    operation(&lhs, &rhs)
                })
                .reduce(|acc, v| v.concat(&acc))
                .ok_or(LLVMExecutorError::MalformedInstruction)
        }

        // TODO: Check out scalable vectors.
        (VectorType { .. }, VectorType { .. }) => {
            todo!()
        }

        // These types should not appear in a binary operation.
        _ => Err(LLVMExecutorError::MalformedInstruction),
    }
}

/// Converts integers, pointers, or vectors.
///
/// Performs a conversion on either (int,int), (ptr,int), (int,ptr), or (vector,vector) with the
/// passed mapping function.
///
/// No type checking is done, if this is of interest they have to be checked before calling this
/// function.
pub(crate) fn convert_to_map<'p, T, F>(state: &LLVMState, ty: &Type, op: T, map: F) -> Result<DExpr>
where
    T: Into<Op<'p>>,
    F: Fn(DExpr, u32) -> DExpr,
{
    let op = op.into();
    let symbol = state.get_expr(op)?;
    let source_ty = state.type_of(&op);

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
            assert!(source_bits * num_elements == symbol.len());

            // Process each element one by one and concatenate the result.
            (0..num_elements)
                .map(|i| {
                    let low = i * source_bits;
                    let high = (i + 1) * source_bits - 1;
                    let symbol = symbol.slice(low, high);
                    map(symbol, target_bits)
                })
                .reduce(|acc, v| v.concat(&acc))
                .ok_or(LLVMExecutorError::MalformedInstruction)
        }

        // TODO: Check if scalable vectors are similar
        (VectorType { .. }, VectorType { .. }) => Err(LLVMExecutorError::UnsupportedInstruction(
            "Scalable vectors".to_owned(),
        )),

        // The other types should not appear for this instruction.
        _ => Err(LLVMExecutorError::MalformedInstruction),
    }
}

/// Cast operand to type `ty`.
///
/// Casting simply reinterprets the bits as a different type. As the system does not return types,
/// this just returns the underlying symbol. The bit widths must match.
pub(crate) fn cast_to<'p, T>(state: &LLVMState, ty: &Type, op: T) -> Result<DExpr>
where
    T: Into<Op<'p>>,
{
    let bv = state.get_expr(op.into())?;
    assert_eq!(bv.len(), state.project.bit_size(ty).unwrap() as u32);
    Ok(bv)
}

pub(crate) fn icmp(
    state: &LLVMState,
    lhs: &Operand,
    rhs: &Operand,
    predicate: IntPredicate,
) -> Result<DExpr> {
    binop(state, lhs, rhs, |lhs, rhs| match predicate {
        IntPredicate::EQ => lhs._eq(&rhs),
        IntPredicate::NE => lhs._ne(&rhs),
        IntPredicate::UGT => lhs.ugt(&rhs),
        IntPredicate::UGE => lhs.ugte(&rhs),
        IntPredicate::ULT => lhs.ult(&rhs),
        IntPredicate::ULE => lhs.ulte(&rhs),
        IntPredicate::SGT => lhs.sgt(&rhs),
        IntPredicate::SGE => lhs.sgte(&rhs),
        IntPredicate::SLT => lhs.slt(&rhs),
        IntPredicate::SLE => lhs.slte(&rhs),
    })
}

use llvm_ir::{IntPredicate, Type};

use super::{get_bit_offset_concrete, Op, Size, ToValue};
use crate::{
    solver::{BinaryOperation, BV},
    traits::{get_byte_offset_concrete, get_byte_offset_symbol},
    vm::{Result, State},
};

pub fn extract_value<'p, T>(state: &mut State<'_>, aggregate: T, indices: &[u32]) -> Result<BV>
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

    let offset_upper_bound = total_offset + ty.size(&state.project).unwrap();

    // Get the value and check that the BV is big enough to accomodate our slice.
    let value = state.get_var(aggregate)?;
    assert!(value.len() >= offset_upper_bound as u32);

    let value = value.slice(total_offset as u32, offset_upper_bound as u32 - 1);
    Ok(value)
}

pub fn gep<'p, T, I>(state: &mut State<'_>, address: T, indices: I, _in_bounds: bool) -> Result<BV>
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
            get_byte_offset_symbol(&curr_ty, &index, state)?
        };

        addr = addr.add(&offset);
        curr_ty = ty;
    }

    Ok(addr)
}

pub fn binop<'p, T>(state: &mut State<'_>, lhs: T, rhs: T, op: BinaryOperation) -> Result<BV>
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

pub fn cast_to<'p, T>(state: &mut State<'_>, ty: &Type, op: T) -> Result<BV>
where
    T: Into<Op<'p>>,
{
    let bv = state.get_var(op.into())?;
    assert_eq!(bv.len(), ty.size(state.project).unwrap() as u32);
    Ok(bv)
}

pub fn icmp<'p, T>(state: &mut State<'_>, lhs: T, rhs: T, predicate: IntPredicate) -> Result<BV>
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

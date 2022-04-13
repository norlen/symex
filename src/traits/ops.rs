use llvm_ir::{IntPredicate, Type};

use super::{Op, Size, ToValue};
use crate::{
    solver::{BinaryOperation, BV},
    vm::{Result, State, VMError},
};

const ENABLE_BOUNDS_CHECK: bool = false;

pub fn gep<'p, T, I>(state: &mut State<'_>, address: T, indices: I, in_bounds: bool) -> Result<BV>
where
    T: Into<Op<'p>>,
    I: Iterator<Item = Op<'p>>,
{
    let address = address.into();
    // Currently, symbolic values are supported except in the case of struct
    // field addressing. This require concretization which could be support
    // for at least a few values.
    //
    // Hence, we check if the current type is a struct. And if it is, the
    // operand is required to be a constant, for now.
    let mut addr = state.get_var(address).unwrap();
    let ptr_size = addr.len();

    let bounds = if ENABLE_BOUNDS_CHECK && in_bounds {
        let ty = state.type_of(&address);
        let size = ty.size(&state.project).unwrap();
        let size = state.solver.bv_from_u64(size, ptr_size);
        let upper_bound = addr.add(&size);

        Some((addr.clone(), upper_bound))
    } else {
        None
    };

    // The offsets modifies the address ptr, and this is the type of what
    // is currently pointed to.
    let mut curr_ty = state.type_of(&address);
    for index in indices {
        let is_struct = matches!(
            curr_ty.as_ref(),
            Type::NamedStructType { .. } | Type::StructType { .. }
        );

        let (offset, ty) = if is_struct {
            // Concrete indexing into a struct.
            let index = index.to_value()?;
            let (offset, ty) = curr_ty.offset_in_bytes(index, state.project)?.unwrap();

            let offset = state.solver.bv_from_u64(offset, ptr_size);

            (offset, ty)
        } else {
            // Symbolic index. We cannot support struct indexing here, since
            // we calculate the offset as size of type * index, which won't
            // offset correctly for structs.
            let index = state.get_var(index)?;
            let index = index.zero_ext(ptr_size);

            let bytes = curr_ty.size(state.project).unwrap();
            let bytes = state.solver.bv_from_u64(bytes, ptr_size);

            let ty = curr_ty.inner_ty(state.project).unwrap();

            let offset = bytes.mul(&index).into();
            (offset, ty)
        };

        addr = addr.add(&offset);
        curr_ty = ty;
    }

    // Check that the target address is in bounds.
    if let Some((lower_bound, upper_bound)) = bounds {
        let is_below = addr.ult(&lower_bound);
        let is_above = addr.ugte(&upper_bound);
        let out_of_bounds = is_below.or(&is_above).into();
        if state.solver.is_sat_with_constraint(&out_of_bounds).unwrap() {
            state.solver.assert(&out_of_bounds);
            let sol = state.solver.get_solutions_for_bv(&addr, 1).unwrap();
            match sol {
                crate::Solutions::None => println!("==== no solution"),
                crate::Solutions::Exactly(n) => println!("==== exact {:?}", n),
                crate::Solutions::AtLeast(n) => println!("==== atleast {:?}", n),
            }
            // panic!("not in bounds");
            return Err(VMError::OutOfBounds);
        }
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

use std::fs::OpenOptions;

use llvm_ir::{Constant, Operand, Type};

use crate::traits::Size;
use crate::{
    solver::{BinaryOperation, BV},
    vm::{Result, State},
};

// trait ToBV {
//     fn to_bv(&self, state: &mut State<'_>) -> BV;
// }

// impl ToBV for &Operand {
//     fn to_bv(&self, state: &mut State<'_>) -> BV {
//         todo!()
//     }
// }

// impl ToBV for &Constant {
//     fn to_bv(&self, state: &mut State<'_>) -> BV {
//         todo!()
//     }
// }

// pub fn binop(state: &mut State<'_>, lhs: &Operand, rhs: &Operand, op: BinaryOperation) -> BV {
//     // TODO: Could check that types match?
//     // let lhs_ty = self.state.type_of(lhs);
//     // let rhs_ty = self.state.type_of(rhs);

//     let lhs_bv = state.get_bv(lhs).unwrap();
//     let rhs_bv = state.get_bv(rhs).unwrap();
//     assert_eq!(lhs_bv.len(), rhs_bv.len());

//     lhs_bv.binary_op(rhs_bv, op)
// }

// pub fn binop_const(
//     state: &mut State<'_>,
//     lhs: &Constant,
//     rhs: &Constant,
//     op: BinaryOperation,
// ) -> Result<BV> {
//     let lhs_bv = state.get_bv_from_constant(lhs)?;
//     let rhs_bv = state.get_bv_from_constant(rhs)?;
//     assert_eq!(lhs_bv.len(), rhs_bv.len());

//     Ok(lhs_bv.binary_op(rhs_bv, op))
// }

// /// Bitcast casts a value to to resulting type without changing any of the
// /// bits.
// ///
// /// Essentially a no-op, the implementation only stores the source value
// /// with the destination name and type.
// ///
// /// Reference: https://llvm.org/docs/LangRef.html#bitcast-to-instruction
// pub fn bitcast(state: &mut State<'_>, op: &Operand, to: &Type) -> Result<BV> {
//     let bv = state.get_bv(op)?;
//     assert_eq!(bv.len() as u64, to.size(&state.project).unwrap());
//     Ok(bv)
// }

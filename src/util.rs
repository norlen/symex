use llvm_ir::{ConstantRef, Operand, Type};

use crate::llvm::{AsConcrete, Size};
use crate::vm::{Result, State, VMError};
use crate::BV;

pub fn gep_op(
    address: &Operand,
    indices: &[Operand],
    in_bounds: bool,
    state: &mut State<'_>,
) -> Result<BV> {
    // Currently, symbolic values are supported except in the case of struct
    // field addressing. This require concretization which could be support
    // for at least a few values.
    //
    // Hence, we check if the current type is a struct. And if it is, the
    // operand is required to be a constant, for now.
    let mut addr = state.get_bv(address).unwrap();
    let ptr_size = addr.len();

    let bounds = if in_bounds {
        let value_ty = state.type_of(address);
        let value_size = value_ty.size(state.project).unwrap();
        let value_size = state.solver.bv_from_u64(value_size, ptr_size);
        let upper_bound = addr.add(&value_size);

        Some((addr.clone(), upper_bound))
    } else {
        None
    };
    println!("startaddr: {:?}", addr);
    println!("bounds: {:?}", bounds);

    // The offsets modifies the address ptr, and this is the type of what
    // is currently pointed to.
    let mut curr_ty = state.type_of(address);
    for index in indices.iter() {
        let is_struct = matches!(
            curr_ty.as_ref(),
            Type::NamedStructType { .. } | Type::StructType { .. }
        );

        let (offset, ty) = if is_struct {
            // Concrete indexing into a struct.
            let index = index.as_concrete().unwrap();
            let (offset, ty) = curr_ty
                .offset_in_bytes(index, state.project)
                .unwrap()
                .unwrap();
            // let index = u64_from_operand(index).unwrap() as usize;
            // let (offset, ty) = get_offset_in_bits(&curr_ty, index, self.project);

            println!("offset: {}", offset);

            let offset = state.solver.bv_from_u64(offset, ptr_size);

            (offset, ty)
        } else {
            // Symbolic index. We cannot support struct indexing here, since
            // we calculate the offset as size of type * index, which won't
            // offset correctly for structs.
            let index = state.get_bv(index).unwrap();
            let index = index.zero_ext(ptr_size);

            let bytes = curr_ty.size(state.project).unwrap();
            let bytes = state.solver.bv_from_u64(bytes, ptr_size);

            let ty = curr_ty.inner_ty(state.project).unwrap();

            let offset = bytes.mul(&index).into();
            println!("offset: {:?}", offset);
            (offset, ty)
        };
        println!("offset: {:?}", offset);

        addr = addr.add(&offset);
        curr_ty = ty;
    }

    println!("---------------------------------------------------------------");
    println!("in_bounds: {}", in_bounds);
    println!("addr: {:?}", addr);
    println!("currty: {:?}", curr_ty);
    // Check that the target address is in bounds.
    // if let Some((lower_bound, upper_bound)) = bounds {
    //     let is_below = addr.ult(&lower_bound);
    //     let is_above = addr.ugte(&upper_bound);
    //     let out_of_bounds = is_below.or(&is_above).into();
    //     if state.solver.is_sat_with_constraint(&out_of_bounds).unwrap() {
    //         out_of_bounds.assert();
    //         let sol = state.solver.get_solutions_for_bv(&addr, 1).unwrap();
    //         match sol {
    //             crate::Solutions::None => println!("==== no solution"),
    //             crate::Solutions::Exactly(n) => println!("==== exact {:?}", n),
    //             crate::Solutions::AtLeast(n) => println!("==== atleast {:?}", n),
    //         }
    //         // panic!("not in bounds");
    //         return Err(VMError::OutOfBounds);
    //     }
    // }

    Ok(addr)
}

pub fn gep_constant(
    address: &ConstantRef,
    indices: &[ConstantRef],
    in_bounds: bool,
    state: &mut State<'_>,
) -> Result<BV> {
    let op = Operand::ConstantOperand(address.clone());
    let indices: Vec<Operand> = indices
        .iter()
        .map(|c| Operand::ConstantOperand(c.clone()))
        .collect();

    gep_op(&op, &indices, in_bounds, state)
}

// pub fn gep(
//     address: &Operand,
//     indices: &[Operand],
//     in_bounds: bool,
//     state: &mut State<'_>,
// ) -> anyhow::Result<BV> {
//     let addr = state.get_bv(address)?;
//     let addr_size = addr.get_width();

//     let mut curr_ty = state.type_of(address);
//     for index in indices.iter() {
//         let is_struct = matches!(
//             curr_ty.as_ref(),
//             Type::NamedStructType { .. } | Type::StructType { .. }
//         );

//         let (offset, ty) = if is_struct {
//             // Concrete indexing into a struct.
//             let index = index.as_concrete().unwrap();
//             let (offset, ty) = curr_ty.offset(index, state.project).unwrap();

//             let offset_bytes = (offset / 8) as u64;
//             let offset = state.solver.bv_from_u64(offset_bytes, addr_size);

//             (offset, ty)
//         } else {
//             let index = state.get_bv(index)?;
//             // let index = index.to_bv(state)?;
//             let index = index.zero_ext(addr_size);

//             let bytes = curr_ty.size_in_bytes(state.project).unwrap().unwrap() as u64;
//             let bytes = state.solver.bv_from_u64(bytes, addr_size);

//             let ty = curr_ty.inner_ty(state.project).unwrap();

//             let offset = bytes.mul(&index).into();
//             (offset, ty)
//         };

//         addr.add(&offset);
//         curr_ty = ty;
//     }

//     Ok(addr)
// }

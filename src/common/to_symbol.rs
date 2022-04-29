//! Converts [llvm_ir::Operand] and [llvm_ir::Constant] to symbols.
//!
//!
use anyhow::anyhow;
use llvm_ir::{Constant, ConstantRef, IntPredicate, Operand, Type};

use super::{convert_to_map, gep, ToValue};
use crate::{
    solver::BV,
    vm::{Result, State, VMError},
};

/// Convert an operand to a symbol.
pub fn operand_to_symbol(state: &State<'_>, operand: &Operand) -> Result<BV> {
    use Operand::*;

    match operand {
        ConstantOperand(c) => const_to_symbol(state, c.as_ref()),
        LocalOperand { name, .. } => state
            .vars
            .get(name)
            .cloned()
            .ok_or_else(|| VMError::LocalNotFound(name.to_string())),
        MetadataOperand => Err(anyhow!("Cannot convert operand {:?} to symbol", operand).into()),
    }
}

/// Convert an operand to a symbol.
///
/// The operand may either reference a variable in which case it cannot be zero sized, since those
/// are not stored (maybe they should?). Otherwise, it's a constant in which case it can be.
pub fn operand_to_symbol_zero_size(state: &State<'_>, operand: &Operand) -> Result<Option<BV>> {
    // Don't think I'll use this? Otherwise the non zero sized can just call this.
    use Operand::*;

    match operand {
        ConstantOperand(c) => const_to_symbol_zero_size(state, c.as_ref()),
        LocalOperand { name, .. } => state
            .vars
            .get(name)
            .cloned()
            .map(Some)
            .ok_or_else(|| VMError::LocalNotFound(name.to_string())),
        MetadataOperand => Err(anyhow!("Cannot convert operand {:?} to symbol", operand).into()),
    }
}

/// Convert a constant to a symbol.
///
/// Requires the final size to not be zero sized. State is required since global references
/// are allowed in constants.
pub fn const_to_symbol(state: &State<'_>, constant: &Constant) -> Result<BV> {
    let value = const_to_symbol_zero_size(state, constant)?;
    value.ok_or(VMError::UnexpectedZeroSize)
}

/// Convert a constant to a symbol that allows the entire thing to be zero sized.
///
/// State is required since global references are allowed in constants.
pub fn const_to_symbol_zero_size(state: &State<'_>, constant: &Constant) -> Result<Option<BV>> {
    use Constant::*;
    match constant {
        // Standard integers of a certain bit width that have a well defined value.
        Int { bits, value } => Ok(Some(state.solver.bv_from_u64(*value, *bits))),

        // `Undef` indicates that the value may have an unspecified bit pattern. It it allowed for
        // all types except `void` and `label`.
        //
        // These should indicate that the program is well behaved no matter the value, thus we put
        // these as unconstrained so we can check for possible errors.
        //
        // Not sure if the generated LLVM does not allow for these errors to happen, but if it does
        // those kind of errors are covered.
        Undef(ty) => {
            let size = state.project.bit_size(ty)?;
            Ok(match size {
                0 => None,
                n => {
                    // TODO: boolector does not like unnamed bitvectors, thus zero allocate this
                    // as well just to get around it.
                    Some(state.solver.bv_zero(n))
                }
            })
        }

        // Both null pointers and the aggregate of zeroes are initialized to zero.
        Null(ty) | AggregateZero(ty) => {
            let size = state.project.bit_size(ty)?;
            Ok(match size {
                0 => None,
                n => Some(state.solver.bv_zero(n as u32)),
            })
        }

        // Array, Vector and Structs can all be converted the same way.
        //
        // Struct members are converted into their respective values one by one, and they have may
        // zero-sized members.
        //
        // For arrays and vectors, it seems weird for those to have zero-sized elements, but if they
        // for some reason do, why not support it.
        //
        // TODO: Check vector types, e.g. a vector <4xi4> <i4 1, i4 2, i4 3, i4 4> is laid out as
        // 0x4321 in little endian systems. May be what I'm doing below.
        Array { elements: e, .. } | Vector(e) | Struct { values: e, .. } => {
            // Process each member as maybe being zero-sized. All of them might be thus the reduce
            // can receive zero elements.
            let values = e
                .iter()
                .filter_map(|c| const_to_symbol_zero_size(state, c).transpose())
                .collect::<Result<Vec<_>>>()?;
            Ok(values.into_iter().reduce(|acc, v| v.concat(&acc)))
        }

        // Global reference should be added and initialized before starting execution.
        //
        // These are always pointers, and as the pointers do not change they are concrete. Thus,
        // they are simply just converted to symbols.
        GlobalReference { name, .. } => {
            let global = state
                .global_references
                .get(name, state.current_loc.module)
                .cloned()
                .ok_or_else(|| VMError::Other(anyhow!("Global ref not found: {:?}", name)))?;

            let addr = state
                .solver
                .bv_from_u64(global.addr, state.project.ptr_size as u32);

            Ok(Some(addr))
        }

        // Binary operations, the operands here cannot be zero sized. So if a zero sized type is
        // encountered it will return a [VMError::UnexpectedZeroSize].
        Add(op) => bin(state, &op.operand0, &op.operand1, BV::add),
        Sub(op) => bin(state, &op.operand0, &op.operand1, BV::sub),
        Mul(op) => bin(state, &op.operand0, &op.operand1, BV::mul),
        UDiv(op) => bin(state, &op.operand0, &op.operand1, BV::udiv),
        SDiv(op) => bin(state, &op.operand0, &op.operand1, BV::sdiv),
        URem(op) => bin(state, &op.operand0, &op.operand1, BV::urem),
        SRem(op) => bin(state, &op.operand0, &op.operand1, BV::srem),
        And(op) => bin(state, &op.operand0, &op.operand1, BV::and),
        Or(op) => bin(state, &op.operand0, &op.operand1, BV::or),
        Xor(op) => bin(state, &op.operand0, &op.operand1, BV::xor),
        Shl(op) => bin(state, &op.operand0, &op.operand1, BV::sll),
        LShr(op) => bin(state, &op.operand0, &op.operand1, BV::srl),
        AShr(op) => bin(state, &op.operand0, &op.operand1, BV::sra),

        // These are conversion from ptr->int or int->ptr. These do not allow zero sized types now.
        PtrToInt(op) => {
            let result = convert_to_map(state, &op.to_type, &op.operand, |symbol, target_size| {
                symbol.resize_unsigned(target_size)
            })?;
            Ok(Some(result))
        }
        IntToPtr(op) => {
            let result = convert_to_map(state, &op.to_type, &op.operand, |symbol, target_size| {
                symbol.resize_unsigned(target_size)
            })?;
            Ok(Some(result))
        }

        // These are all just casts between types. If the constants they have inside are zero-sized
        // the result will be `None`. Not sure it makes sense to allow this though.
        BitCast(op) => const_cast(state, &op.to_type, &op.operand),
        AddrSpaceCast(op) => const_cast(state, &op.to_type, &op.operand),

        // ICMP compares the two values using logical operators, returning a bool. For the
        // comparisons zero-sized types are not allowed.
        ICmp(op) => {
            let lhs = const_to_symbol_zero_size(state, &op.operand0)?
                .ok_or(VMError::UnexpectedZeroSize)?;
            let rhs = const_to_symbol_zero_size(state, &op.operand1)?
                .ok_or(VMError::UnexpectedZeroSize)?;

            use IntPredicate::*;
            let result = match op.predicate {
                EQ => lhs.eq(&rhs),
                NE => lhs.ne(&rhs),
                UGT => lhs.ugt(&rhs),
                UGE => lhs.ugte(&rhs),
                ULT => lhs.ult(&rhs),
                ULE => lhs.ulte(&rhs),
                SGT => lhs.sgt(&rhs),
                SGE => lhs.sgte(&rhs),
                SLT => lhs.slt(&rhs),
                SLE => lhs.slte(&rhs),
            };
            Ok(Some(result))
        }

        // Truncate operand to target type. The target type *must* be smaller than the current.
        Trunc(op) => {
            let result = convert_to_map(state, &op.to_type, &op.operand, |symbol, target_size| {
                symbol.slice(0, target_size - 1)
            })?;
            Ok(Some(result))
        }

        // Zero extend operand to target type. The current value must have a smaller bit width
        // compared to the target type.
        ZExt(op) => {
            let result = convert_to_map(state, &op.to_type, &op.operand, |symbol, target_size| {
                symbol.zero_ext(target_size)
            })?;
            Ok(Some(result))
        }

        // Sign extend operand to target type. The current value must have a smaller bit width
        // compared to the target type.
        SExt(op) => {
            let result = convert_to_map(state, &op.to_type, &op.operand, |symbol, target_size| {
                symbol.sign_ext(target_size)
            })?;
            Ok(Some(result))
        }

        // Extract a scalar element from a vector at a specific index.
        //
        // The first operand must be of vector type. The second operand is the integer index.
        ExtractElement(op) => {
            if let Vector(elements) = op.vector.as_ref() {
                let index = op.index.to_value()? as usize;
                let value = elements.get(index).ok_or(VMError::MalformedInstruction)?;

                const_to_symbol_zero_size(state, value)
            } else {
                Err(VMError::MalformedInstruction)
            }
        }

        // Insert an element at a specific index in a vector.
        //
        // First operand is the vector, second is a scalar where the type must match the vector
        // type. The third operand is the index where the scalar should be inserted.
        InsertElement(op) => {
            if let Vector(elements) = op.vector.as_ref() {
                let index = op.index.to_value()? as usize;
                if index >= elements.len() {
                    return Err(VMError::MalformedInstruction);
                }

                let mut elements = elements.clone();
                elements[index] = op.element.clone();

                const_to_symbol_zero_size(state, &Vector(elements))
            } else {
                Err(VMError::MalformedInstruction)
            }
        }

        // Shuffle vector constructs a new vector from a pair of vectors and a mask, returning a
        // vector of the same size as the inputs.
        //
        // The first two operand are the source vector. They are numbered from left-to-right. The
        // third is the shuffle vector which decides which element to pick.
        ShuffleVector(op) => match (op.operand0.as_ref(), op.operand1.as_ref(), op.mask.as_ref()) {
            (Vector(e0), Vector(e1), Vector(mask)) => {
                let elements: Vec<ConstantRef> =
                    e0.iter().cloned().chain(e1.iter().cloned()).collect();

                let mut constants = Vec::new();
                for index in mask.iter() {
                    let index = index.to_value()? as usize;
                    let constant = elements
                        .get(index)
                        .cloned()
                        .ok_or(VMError::MalformedInstruction)?;

                    constants.push(constant);
                }

                const_to_symbol_zero_size(state, &Vector(constants))
            }
            _ => Err(VMError::MalformedInstruction),
        },

        // Extract value from a struct or array.
        //
        // First operand is a struct or array, the second operand is the index.
        //
        // For this instruction:
        // - At least one index must exist.
        // - Both struct indices and array indices must be in bounds.
        ExtractValue(op) => {
            let mut constant = &op.aggregate;
            for index in op.indices.iter().copied() {
                match constant.as_ref() {
                    Struct {
                        values: elements, ..
                    }
                    | Array { elements, .. } => {
                        constant = elements
                            .get(index as usize)
                            .ok_or(VMError::MalformedInstruction)?;
                    }
                    _ => return Err(VMError::MalformedInstruction),
                }
            }

            const_to_symbol_zero_size(state, constant)
        }

        // Insert value to a member field of an array or struct.
        //
        // First operand is a struct or array. Second operand is a first-class value to insert.
        // Followed by the indices.
        InsertValue(op) => {
            // Helper to recursively replace the chosen value in a struct/array. Goes into the
            // aggregate until all indices have been exhausted.
            fn replace_value(
                current: Constant,
                replace_with: Constant,
                indices: &[u32],
                i: usize,
            ) -> Result<Constant> {
                if i == indices.len() {
                    // When we hit this we have exhausted all indices, and this is what we should get.
                    //
                    // Might want to check that this type matches the current one, but that just
                    // means the IR is malformed, so not top priority.
                    return Ok(replace_with);
                }

                match current {
                    Array { elements: e, .. } | Struct { values: e, .. } => {
                        let index = indices[i] as usize;
                        let element = e.get(index).ok_or(VMError::MalformedInstruction)?;

                        replace_value(element.as_ref().clone(), replace_with, indices, i + 1)
                    }
                    _ => Err(VMError::MalformedInstruction),
                }
            }

            let constant = replace_value(
                op.aggregate.as_ref().clone(),
                op.element.as_ref().clone(),
                &op.indices,
                0,
            )?;
            const_to_symbol_zero_size(state, &constant)
        }

        // Calculate address of a sub element in an aggregate.
        GetElementPtr(op) => gep(
            state,
            &op.address,
            op.indices.iter().map(|c| c.into()),
            op.in_bounds,
        )
        .map(Some),

        // Select one operand or the other without branches.
        Select(op) => {
            // Assume the condition is constant, if this does not hold then the regular instruction
            // operation should be used.
            let condition = op.condition.to_value()?;
            if condition == 0 {
                const_to_symbol_zero_size(state, &op.false_value)
            } else {
                const_to_symbol_zero_size(state, &op.true_value)
            }
        }

        // Floating point is currently not supported.
        Float(_) | FAdd(_) | FSub(_) | FMul(_) | FDiv(_) | FRem(_) | FPTrunc(_) | FPExt(_)
        | FPToUI(_) | FPToSI(_) | UIToFP(_) | SIToFP(_) | FCmp(_) => {
            Err(VMError::UnsupportedInstruction("Floating point".to_owned()))
        }

        // TODO
        Poison(_) => todo!(),
        BlockAddress => todo!(),
        TokenNone => todo!(),
    }
}

/// Just reinterpret the underlying bytes as a different type. In this case it just means returning
/// the underlying symbol.
fn const_cast(state: &State<'_>, ty: &Type, constant: &ConstantRef) -> Result<Option<BV>> {
    let result = const_to_symbol_zero_size(state, constant)?.map(|bv| {
        assert_eq!(bv.len(), state.project.bit_size(ty).unwrap() as u32);
        bv
    });
    Ok(result)
}

/// Helper to perform a binary operation on constants.
///
/// This is similar to the binary helper for operands in the regular instructions, but as these
/// are always constants this is just a simpler version.
///
/// Requires that `lhs` and `rhs` are not be zero sized.
fn bin<F>(
    state: &State<'_>,
    lhs: &ConstantRef,
    rhs: &ConstantRef,
    operation: F,
) -> Result<Option<BV>>
where
    F: Fn(&BV, &BV) -> BV,
{
    // TODO: May want to replace this with the regular binary op thing from instructions.
    use Constant::*;
    match (lhs.as_ref(), rhs.as_ref()) {
        // Assume the constants are similar as in the regular instruction, in that only integers,
        // floating points and vectors of those are allowed.
        (Int { .. }, Int { .. }) => {
            let lhs = const_to_symbol(state, lhs)?;
            let rhs = const_to_symbol(state, rhs)?;

            let result = operation(&lhs, &rhs);
            Ok(Some(result))
        }

        // For vectors to the operation on a per element basis.
        (Vector(e0), Vector(e1)) => {
            if e0.len() != e1.len() {
                return Err(VMError::MalformedInstruction);
            }
            let elements = e0.iter().zip(e1.iter());

            // Perform the operations per element and concatenate.
            let result = elements
                .map(|(lhs, rhs)| {
                    let lhs = const_to_symbol(state, lhs);
                    let rhs = const_to_symbol(state, rhs);
                    lhs.and_then(|lhs| rhs.map(|rhs| lhs.add(&rhs)))
                })
                .reduce(|acc, v| Ok(v?.concat(&acc?)))
                .ok_or(VMError::MalformedInstruction)??;

            Ok(Some(result))
        }

        // Not supported yet.
        (Float(_), Float(_)) => Err(VMError::UnsupportedInstruction("Floating point".to_owned())),

        // These types should not appear here.
        _ => Err(VMError::MalformedInstruction),
    }
}

#[cfg(test)]
mod tests {
    // TODO: Quite hard to setup tests, as the constants are not self-contained. They need access
    // to the state since they can have global references.
    //
    // Could create a sample file with a lot of constants, and try to evaluate those.
}

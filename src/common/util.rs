use llvm_ir::Operand;
use log::warn;

use crate::{
    common::ToValue,
    vm::{Result, State, VMError},
    Solutions,
};

pub fn get_u64_solution_from_operand(state: &State<'_>, op: &Operand) -> Result<u64> {
    let symbol = match op.to_value() {
        Ok(value) => return Ok(value),
        _ => state.get_var(op)?,
    };

    match state.solver.get_solutions_for_bv(&symbol, 1)? {
        Solutions::None => Err(VMError::Unsat),
        Solutions::Exactly(solutions) => Ok(solutions[0].as_u64().unwrap()),
        Solutions::AtLeast(solutions) => {
            warn!("Multiple solutions exist for {op:?}");
            Ok(solutions[0].as_u64().unwrap())
        }
    }
}

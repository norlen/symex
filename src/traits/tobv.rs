use anyhow::anyhow;
use llvm_ir::{Constant, Operand};

use crate::{
    solver::{BinaryOperation, BV},
    vm::{AllocationType, Result, State, VMError},
};

use super::*;

impl ToBV for &Operand {
    fn to_bv(&self, state: &mut State<'_>) -> Result<BV> {
        use Operand::*;

        match self {
            ConstantOperand(c) => c.as_ref().to_bv(state),
            LocalOperand { name, .. } => Ok(state.vars.get(name).cloned().unwrap()),
            MetadataOperand => Err(anyhow!("Cannot convert operand {:?} to BV", self).into()),
        }
    }

    fn try_to_bv(&self, _state: &mut State<'_>) -> Result<Option<BV>> {
        todo!()
    }
}

impl ToBV for &Constant {
    fn to_bv(&self, state: &mut State<'_>) -> Result<BV> {
        use Constant::*;

        match self {
            Int { bits, value } => Ok(state.solver.bv_from_u64(*value, *bits)),
            Float(_) => todo!(),
            Null(ty) | AggregateZero(ty) => {
                let size = ty
                    .size(&state.project)
                    .ok_or_else(|| VMError::UnexpectedZeroSize)?;
                Ok(state.solver.bv_zero(size as u32))
            }
            Struct {
                values: elements, ..
            }
            | Array { elements, .. }
            | Vector(elements) => elements
                .iter()
                .map(|c| c.as_ref().to_bv(state))
                .reduce(|acc, v| Ok(v?.concat(&acc?)))
                .ok_or_else(|| VMError::UnexpectedZeroSize)?,
            Undef(_) => todo!(),
            Poison(_) => todo!(),
            BlockAddress => todo!(),
            GlobalReference { name, .. } => {
                if let Some(global) = state.get_global(name).cloned() {
                    match &global.kind {
                        AllocationType::Variable(v) => {
                            if !v.initialized.get() {
                                let value = state.get_var(&v.initializer)?;
                                state.mem.write(&global.addr_bv, value)?;
                                v.initialized.set(true);
                            }
                        }
                        AllocationType::Function(_) => {}
                    }
                    Ok(global.addr_bv.clone())
                } else {
                    panic!("global ref not found, {:?}", name);
                }
            }
            TokenNone => todo!(),
            Add(op) => binop(state, &op.operand0, &op.operand1, BinaryOperation::Add),
            Sub(op) => binop(state, &op.operand0, &op.operand1, BinaryOperation::Sub),
            Mul(op) => binop(state, &op.operand0, &op.operand1, BinaryOperation::Mul),
            UDiv(op) => binop(state, &op.operand0, &op.operand1, BinaryOperation::UDiv),
            SDiv(op) => binop(state, &op.operand0, &op.operand1, BinaryOperation::SDiv),
            URem(op) => binop(state, &op.operand0, &op.operand1, BinaryOperation::URem),
            SRem(op) => binop(state, &op.operand0, &op.operand1, BinaryOperation::SRem),
            And(op) => binop(state, &op.operand0, &op.operand1, BinaryOperation::And),
            Or(op) => binop(state, &op.operand0, &op.operand1, BinaryOperation::Or),
            Xor(op) => binop(state, &op.operand0, &op.operand1, BinaryOperation::Xor),
            Shl(_) => todo!(),
            LShr(_) => todo!(),
            AShr(_) => todo!(),
            FAdd(_) => todo!(),
            FSub(_) => todo!(),
            FMul(_) => todo!(),
            FDiv(_) => todo!(),
            FRem(_) => todo!(),
            ExtractElement(_) => todo!(),
            InsertElement(_) => todo!(),
            ShuffleVector(_) => todo!(),
            ExtractValue(_) => todo!(),
            InsertValue(_) => todo!(),
            GetElementPtr(op) => gep(
                state,
                &op.address,
                op.indices.iter().map(|c| c.into()),
                op.in_bounds,
            ),
            Trunc(_) => todo!(),
            ZExt(_) => todo!(),
            SExt(_) => todo!(),
            FPTrunc(_) => todo!(),
            FPExt(_) => todo!(),
            FPToUI(_) => todo!(),
            FPToSI(_) => todo!(),
            UIToFP(_) => todo!(),
            SIToFP(_) => todo!(),
            PtrToInt(op) => cast_to(state, &op.to_type, &op.operand),
            IntToPtr(op) => cast_to(state, &op.to_type, &op.operand),
            BitCast(op) => cast_to(state, &op.to_type, &op.operand),
            AddrSpaceCast(op) => cast_to(state, &op.to_type, &op.operand),
            ICmp(op) => icmp(state, &op.operand0, &op.operand1, op.predicate),
            FCmp(_) => todo!(),
            Select(op) => {
                // TODO: CHCKME
                let condition = op.condition.to_value()?;
                if condition == 1 {
                    state.get_var(&op.true_value)
                } else {
                    state.get_var(&op.false_value)
                }
            }
        }
    }

    fn try_to_bv(&self, _state: &mut State<'_>) -> Result<Option<BV>> {
        todo!()
    }
}

use llvm_ir::{instruction, terminator, Instruction, Terminator, Type};
use tracing::{debug, trace, warn};

use super::{LLVMExecutor, TerminatorResult};
use crate::executor::llvm::{
    binop, cast_to, convert_to_map, gep, icmp, FunctionType, LLVMExecutorError, Result, ReturnValue,
};
use crate::llvm::{extract_value, get_element_offset};
use crate::memory::Memory;
use crate::smt::{DExpr, SolverError};
use crate::smt::{Expression, Solver, SolverContext};

pub(super) struct LLVMInstruction;

impl<'p> LLVMInstruction {
    /// Process a single LLVM IR instruction.
    pub(super) fn process_instruction(
        e: &mut LLVMExecutor<'p>,
        instr: &'p Instruction,
    ) -> Result<()> {
        match &instr {
            Instruction::Load(i) => Self::load(e, i),
            Instruction::Store(i) => Self::store(e, i),
            Instruction::Alloca(i) => Self::alloca(e, i),
            Instruction::Add(i) => Self::add(e, i),
            Instruction::Sub(i) => Self::sub(e, i),
            Instruction::Mul(i) => Self::mul(e, i),
            Instruction::UDiv(i) => Self::udiv(e, i),
            Instruction::SDiv(i) => Self::sdiv(e, i),
            Instruction::URem(i) => Self::urem(e, i),
            Instruction::SRem(i) => Self::srem(e, i),
            Instruction::And(i) => Self::and(e, i),
            Instruction::Or(i) => Self::or(e, i),
            Instruction::Xor(i) => Self::xor(e, i),
            Instruction::Shl(i) => Self::shl(e, i),
            Instruction::LShr(i) => Self::lshr(e, i),
            Instruction::AShr(i) => Self::ashr(e, i),
            Instruction::FAdd(i) => Self::fadd(e, i),
            Instruction::FSub(i) => Self::fsub(e, i),
            Instruction::FMul(i) => Self::fmul(e, i),
            Instruction::FDiv(i) => Self::fdiv(e, i),
            Instruction::FRem(i) => Self::frem(e, i),
            Instruction::FNeg(i) => Self::fneg(e, i),
            Instruction::ExtractElement(i) => Self::extractelement(e, i),
            Instruction::InsertElement(i) => Self::insertelement(e, i),
            Instruction::ShuffleVector(i) => Self::shufflevector(e, i),
            Instruction::ExtractValue(i) => Self::extractvalue(e, i),
            Instruction::InsertValue(i) => Self::insertvalue(e, i),
            Instruction::Fence(i) => Self::fence(e, i),
            Instruction::CmpXchg(i) => Self::cmpxchg(e, i),
            Instruction::AtomicRMW(i) => Self::atomicrmw(e, i),
            Instruction::GetElementPtr(i) => Self::getelementptr(e, i),
            Instruction::Trunc(i) => Self::trunc(e, i),
            Instruction::ZExt(i) => Self::zext(e, i),
            Instruction::SExt(i) => Self::sext(e, i),
            Instruction::FPTrunc(i) => Self::fptrunc(e, i),
            Instruction::FPExt(i) => Self::fpext(e, i),
            Instruction::FPToUI(i) => Self::fptoui(e, i),
            Instruction::FPToSI(i) => Self::fptosi(e, i),
            Instruction::UIToFP(i) => Self::uitofp(e, i),
            Instruction::SIToFP(i) => Self::sitofp(e, i),
            Instruction::PtrToInt(i) => Self::ptrtoint(e, i),
            Instruction::IntToPtr(i) => Self::inttoptr(e, i),
            Instruction::BitCast(i) => Self::bitcast(e, i),
            Instruction::AddrSpaceCast(i) => Self::addrspacecast(e, i),
            Instruction::ICmp(i) => Self::icmp(e, i),
            Instruction::FCmp(i) => Self::fcmp(e, i),
            Instruction::Phi(i) => Self::phi(e, i),
            Instruction::Select(i) => Self::select(e, i),
            Instruction::Freeze(i) => Self::freeze(e, i),
            Instruction::Call(i) => Self::call(e, i),
            Instruction::VAArg(i) => Self::va_arg(e, i),
            Instruction::LandingPad(i) => Self::landingpad(e, i),
            Instruction::CatchPad(i) => Self::catchpad(e, i),
            Instruction::CleanupPad(i) => Self::cleanuppad(e, i),
        }
    }

    /// Process a single LLVM IR terminator instruction.
    pub(super) fn process_terminator(
        e: &mut LLVMExecutor<'p>,
        terminator: &Terminator,
    ) -> Result<TerminatorResult> {
        match terminator {
            Terminator::Ret(i) => Self::ret(e, i),
            Terminator::Br(i) => Self::br(e, i),
            Terminator::CondBr(i) => Self::condbr(e, i),
            Terminator::Switch(i) => Self::switch(e, i),
            Terminator::IndirectBr(i) => Self::indirectbr(e, i),
            Terminator::Invoke(i) => Self::invoke(e, i),
            Terminator::Resume(i) => Self::resume(e, i),
            Terminator::Unreachable(i) => Self::unreachable(e, i),
            Terminator::CleanupRet(i) => Self::cleanupret(e, i),
            Terminator::CatchRet(i) => Self::catchret(e, i),
            Terminator::CatchSwitch(i) => Self::catchswitch(e, i),
            Terminator::CallBr(i) => Self::callbr(e, i),
        }
    }

    // ---------------------------------------------------------------------------------------------
    // Terminator Instructions
    // ---------------------------------------------------------------------------------------------

    /// Returns control back to the caller, and optionally a return value.
    fn ret(e: &mut LLVMExecutor<'p>, instr: &terminator::Ret) -> Result<TerminatorResult> {
        debug!("{}", instr);
        let value = if let Some(op) = &instr.return_operand {
            let value = e.state.get_expr(op)?;
            Some(value)
        } else {
            None
        };

        // When returning the variable scope has to be destroyed.
        //
        // However, we may not reach this in the case of errors earlier, but that does not matter
        // since if an error is encountered the path is considered dead, and cannot be resumed.
        // e.state.stack_frames.pop();
        // TODO: Arent' I doing this in the return of functions? HOwever, this may fuck shit up
        // when the actual call fn stuff isn't done.

        Ok(TerminatorResult::Return(value))
    }

    /// Unconditionally jumps to a basic block inside the current function.
    fn br(e: &mut LLVMExecutor<'p>, instr: &terminator::Br) -> Result<TerminatorResult> {
        debug!("{}", instr);
        e.branch(&instr.dest)
    }

    /// Jumps to another basic block inside the current function.
    ///
    /// This is the conditional version, so it can pick between two basic blocks. It will evaluate
    /// the condition and check if the condition can be `true`, `false`, or both. If more than one
    /// path can be selected a save point is created.
    ///
    /// If the condition cannot be either `true` or `false` [LLVMExecutorError::Unsat] is returned.
    fn condbr(e: &mut LLVMExecutor<'p>, instr: &terminator::CondBr) -> Result<TerminatorResult> {
        debug!("{}", instr);

        let cond = e.state.get_expr(&instr.condition)?.simplify();
        if let Some(c) = cond.get_constant() {
            // TODO: Add get_constnat_bool
            if c > 0 {
                return e.branch(&instr.true_dest);
            } else {
                return e.branch(&instr.false_dest);
            }
        }
        let true_possible = e.state.constraints.is_sat_with_constraint(&cond)?;
        let false_possible = e.state.constraints.is_sat_with_constraint(&cond.not())?;

        let target = match (true_possible, false_possible) {
            (true, true) => {
                // Explore `true` path, and save `false` path for later.
                e.save_path(&instr.false_dest, Some(cond.not()))?;
                e.state.constraints.assert(&cond);
                Ok(&instr.true_dest)
            }
            (true, false) => Ok(&instr.true_dest),
            (false, true) => Ok(&instr.false_dest),
            (false, false) => Err(SolverError::Unsat),
        }?;

        e.branch(target)
    }

    /// Jump to any number of basic blocks inside the current function.
    ///
    /// A more general version compared to `condbr`. Multiple conditions with jump targets can be
    /// specified, with a default that is taken if none of the conditions apply.
    ///
    /// It will check which conditions are satisfiable, and create save points for these. It will
    /// then branch to the first `true` condition it found.
    fn switch(e: &mut LLVMExecutor<'p>, instr: &terminator::Switch) -> Result<TerminatorResult> {
        debug!("{}", instr);
        let value = e.state.get_expr(&instr.operand).unwrap();

        // The condition for the default term in the switch. The default case is built such that
        //   C = true ^ (val != path_cond_1) ^ (val != path_cond_2) ^ ...
        // So if the default one is the only path, we'll still explore.
        let mut default_cond = e.state.ctx.from_bool(true);

        let mut paths = Vec::new();

        // Check if any of the non-default cases can be reached.
        for (constant, target) in instr.dests.iter() {
            let path_cond = e.state.get_expr(constant).unwrap();

            // Build default condition.
            default_cond = default_cond.and(&value._ne(&path_cond));

            let cond = value._eq(&path_cond);
            if e.state.constraints.is_sat_with_constraint(&cond).unwrap() {
                debug!("switch: path {} possible", target);
                paths.push((target, cond));
            }
        }

        // Check if the default case can be reached.
        if e.state
            .constraints
            .is_sat_with_constraint(&default_cond)
            .unwrap()
        {
            debug!("switch: default path possible");
            paths.push((&instr.default_dest, default_cond));
        }

        // Save backtracking points for all paths except one.
        for (target, cond) in paths.iter().skip(1).cloned() {
            e.save_path(target, Some(cond))?;
        }

        // Jump the the one path that didn't get saved as a backtracking point.
        if let Some((target, cond)) = paths.first() {
            e.state.constraints.assert(cond);
            e.branch(target)
        } else {
            // Should never happen, since if we have no paths at all, the
            // default condition should be just `true`.
            unreachable!();
        }
    }

    /// Indirect branch to a label in the current function. The target is an address derived from
    /// a `BlockAddress`.
    ///
    /// This type is not supported in llvm-ir yet. Thus this instruction is unsupported.
    fn indirectbr(
        _e: &mut LLVMExecutor<'p>,
        instr: &terminator::IndirectBr,
    ) -> Result<TerminatorResult> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction(
            "indirectbr".to_owned(),
        ))
    }

    /// Call the specified function with support for resuming at an exception label.
    ///
    /// If the function returns normally it resumes execution at the `normal` label, if not
    /// execution is resumed at the `exception` label.
    fn invoke(e: &mut LLVMExecutor<'p>, instr: &terminator::Invoke) -> Result<TerminatorResult> {
        debug!("{}", instr);
        // Exception handling is not supported, to this is bascially the same as the regular call.
        //
        // When execptions are supported, these should be caught and the interpreter should then
        // instead jump to the exception label.

        let current_module = e.state.stack_frames.last().unwrap().location.module;
        let function_names = e.resolve_function(&instr.function)?;
        debug!("resolved functions: {:?}", &function_names);
        if function_names.len() > 1 {
            panic!("Multiple symblic function pointers not supported yet");
        }
        let name = function_names.first().unwrap();
        let function = e.project.get_function(&name, current_module)?;

        let return_value = match function {
            FunctionType::Function { function, module } => {
                let arguments = instr
                    .arguments
                    .iter()
                    .map(|(op, _)| e.state.get_expr(op))
                    .collect::<Result<Vec<_>>>()?;

                e.call_fn(module, function, arguments)?
            }
            FunctionType::Hook(hook) => {
                let args: Vec<_> = instr.arguments.iter().map(|(op, _)| op).collect();
                hook(e, &args)?
            }
            FunctionType::Intrinsic(intrinsic) => {
                let args: Vec<_> = instr.arguments.iter().map(|(op, _)| op).collect();
                intrinsic(e, &args)?
            }
        };

        let name = instr.result.clone();
        match return_value {
            ReturnValue::Value(symbol) => e.assign_register(name, symbol)?,
            ReturnValue::Void => {}
        }

        e.branch(&instr.return_label)
    }

    fn callbr(_e: &mut LLVMExecutor<'p>, instr: &terminator::CallBr) -> Result<TerminatorResult> {
        debug!("{}", instr);
        todo!()
    }

    fn resume(_e: &mut LLVMExecutor<'p>, instr: &terminator::Resume) -> Result<TerminatorResult> {
        debug!("{}", instr);
        todo!()
    }

    fn catchswitch(
        _e: &mut LLVMExecutor<'p>,
        instr: &terminator::CatchSwitch,
    ) -> Result<TerminatorResult> {
        debug!("{}", instr);
        todo!()
    }

    fn catchret(
        _e: &mut LLVMExecutor<'p>,
        instr: &terminator::CatchRet,
    ) -> Result<TerminatorResult> {
        debug!("{}", instr);
        todo!()
    }

    fn cleanupret(
        _e: &mut LLVMExecutor<'p>,
        instr: &terminator::CleanupRet,
    ) -> Result<TerminatorResult> {
        debug!("{}", instr);
        todo!()
    }

    /// Unreachable returns an [`LLVMExecutorError::UnreachableInstruction`] error.
    ///
    /// The `unreachable` instruction as it name says, should be unreachable. If this is called, it
    /// is most likely an error in the interpreter.
    fn unreachable(
        _e: &mut LLVMExecutor<'p>,
        instr: &terminator::Unreachable,
    ) -> Result<TerminatorResult> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnreachableInstruction)
    }

    // ---------------------------------------------------------------------------------------------
    // Unary Operations
    // ---------------------------------------------------------------------------------------------

    /// Negates the value of the operand.
    ///
    /// The value must be a floating point type, or a vector of floating points.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn fneg(_e: &mut LLVMExecutor<'p>, instr: &instruction::FNeg) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction("fneg".to_owned()))
    }

    // ---------------------------------------------------------------------------------------------
    // Binary Operations
    //
    // Some binary operations support `nuw` (no unsigned wrap) or `nsw` (no signed wrap). If one is
    // present, the result is a poison value on unsigned/signed overflow.
    //
    // Division supports `exact` which gives a poison value if the first operand is not a multiple
    // of the second one.
    //
    // However, these are not exposed in `llvm_ir` so I cannot access these.
    // ---------------------------------------------------------------------------------------------

    /// Calculate the sum of two integers or two vectors of integers.
    ///
    /// Both arguments must have the same types and must be integers or vectors of integers. On
    /// unsigned overflow the result is mod 2^n, where n is the size in bits.
    fn add(e: &mut LLVMExecutor<'p>, instr: &instruction::Add) -> Result<()> {
        debug!("{}", instr);
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::add)?;
        e.assign_result(instr, result)
    }

    /// Calculate the sum of two floating points or two vectors of floating points.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn fadd(_e: &mut LLVMExecutor<'p>, instr: &instruction::FAdd) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction("fadd".to_owned()))
    }

    /// Calculate the difference of two integers or two vectors of integers.
    ///
    /// Both arguments must have the same types and must be integers or vectors of integers. On
    /// unsigned overflow the result is mod 2^n, where n is the size in bits.
    fn sub(e: &mut LLVMExecutor<'p>, instr: &instruction::Sub) -> Result<()> {
        debug!("{}", instr);
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::sub)?;
        e.assign_result(instr, result)
    }

    /// Calculate the difference of two floating points or two vectors of floating points.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn fsub(_e: &mut LLVMExecutor<'p>, instr: &instruction::FSub) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction("fsub".to_owned()))
    }

    /// Calculates the product of two integers or two vectors of integers.
    ///
    /// Both arguments must have the same types and must be integers or vectors of integers. On
    /// unsigned overflow the result is mod 2^n, where n is the size in bits.
    fn mul(e: &mut LLVMExecutor<'p>, instr: &instruction::Mul) -> Result<()> {
        debug!("{}", instr);
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::mul)?;
        e.assign_result(instr, result)
    }

    /// Calculates the product of two floating points or two vectors of floating points.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn fmul(_e: &mut LLVMExecutor<'p>, instr: &instruction::FMul) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction("fmul".to_owned()))
    }

    /// Calculate the quotient of two integers or two vectors of integers.
    ///
    /// Returns the unsigned quotient of the operands. The denominator cannot be zero.
    fn udiv(e: &mut LLVMExecutor<'p>, instr: &instruction::UDiv) -> Result<()> {
        debug!("{}", instr);
        // TODO: We cannot divide by zero here, we may want to provide an analysis for checking that.
        // However, vectors are supported so this should not be done here. Instead it should be
        // done elsewhere I think? Or we could change it to provide a map, and then we can check
        // it ourselves.
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::udiv)?;
        e.assign_result(instr, result)
    }

    /// Calculate the quotient of two integers or two vectors of integers.
    ///
    /// Returns the signed quotient of the operands. The denominator cannot be zero. Overflow here
    /// also leads to undefined behavior.
    fn sdiv(e: &mut LLVMExecutor<'p>, instr: &instruction::SDiv) -> Result<()> {
        debug!("{}", instr);
        // TODO: Apart from div by zero (see above). The overflow could also be checked I guess,
        // example in docs is 32-bit div with -2147483648 by -1, this may be the only case. i.e.
        // INTx::MIN / -1
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::sdiv)?;
        e.assign_result(instr, result)
    }

    /// Calculate the quotient of two floating points or two vectors of floating points.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn fdiv(_e: &mut LLVMExecutor<'p>, instr: &instruction::FDiv) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction("fdiv".to_owned()))
    }

    /// Calculate the remainder from unsigned division of two integers or a vector of integers.
    ///
    /// The operation performs a division so the denominator cannot be zero.
    fn urem(e: &mut LLVMExecutor<'p>, instr: &instruction::URem) -> Result<()> {
        debug!("{}", instr);
        // TODO: Check div by zero.
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::urem)?;
        e.assign_result(instr, result)
    }

    /// Calculate the remainder from signed division of two integers or two vector of integers.
    ///
    /// The operation performs a division so the denominator cannot be zero.
    fn srem(e: &mut LLVMExecutor<'p>, instr: &instruction::SRem) -> Result<()> {
        debug!("{}", instr);
        // TODO: Check div by zero.
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::srem)?;
        e.assign_result(instr, result)
    }

    /// Calculate the remainder from division of two floating points or two vectors of floating
    /// points.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn frem(_e: &mut LLVMExecutor<'p>, instr: &instruction::FRem) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction("frem".to_owned()))
    }

    // ---------------------------------------------------------------------------------------------
    // Bitwise Binary Operations
    // ---------------------------------------------------------------------------------------------

    /// Logical shift the first operand left by the number of bits specified by the second operand.
    ///
    /// Both parameters must be two integers or two vectors of integers. The second operand is
    /// treated as having unsigned ints.
    fn shl(e: &mut LLVMExecutor<'p>, instr: &instruction::Shl) -> Result<()> {
        // TODO: There are a couple ways to get poison values. Read more about those.
        debug!("{}", instr);
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::sll)?;
        e.assign_result(instr, result)
    }

    /// Logical shift left.
    fn lshr(e: &mut LLVMExecutor<'p>, instr: &instruction::LShr) -> Result<()> {
        // TODO: There are a couple ways to get poison values. Read more about those.
        debug!("{}", instr);
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::srl)?;
        e.assign_result(instr, result)
    }

    /// Arithmetic shift right.
    fn ashr(e: &mut LLVMExecutor<'p>, instr: &instruction::AShr) -> Result<()> {
        // TODO: There are a couple ways to get poison values. Read more about those.
        debug!("{}", instr);
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::sra)?;
        e.assign_result(instr, result)
    }

    /// Bitwise logical and.
    fn and(e: &mut LLVMExecutor<'p>, instr: &instruction::And) -> Result<()> {
        debug!("{}", instr);
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::and)?;
        e.assign_result(instr, result)
    }

    /// Bitwise logical or.
    fn or(e: &mut LLVMExecutor<'p>, instr: &instruction::Or) -> Result<()> {
        debug!("{}", instr);
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::or)?;
        e.assign_result(instr, result)
    }

    /// Bitwise logical xor.
    fn xor(e: &mut LLVMExecutor<'p>, instr: &instruction::Xor) -> Result<()> {
        debug!("{}", instr);
        let result = binop(&e.state, &instr.operand0, &instr.operand1, DExpr::xor)?;
        e.assign_result(instr, result)
    }

    // ---------------------------------------------------------------------------------------------
    // Vector Operations
    // ---------------------------------------------------------------------------------------------

    fn extractelement(_: &mut LLVMExecutor<'p>, instr: &instruction::ExtractElement) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn insertelement(_: &mut LLVMExecutor<'p>, instr: &instruction::InsertElement) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn shufflevector(_: &mut LLVMExecutor<'p>, instr: &instruction::ShuffleVector) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    // ---------------------------------------------------------------------------------------------
    // Aggregate Operations
    // ---------------------------------------------------------------------------------------------

    /// Extract value from an aggregate value.
    ///
    /// First operand should be either a struct or an array. The other operands are the indices
    /// that specify which value should be extracted.
    fn extractvalue(e: &mut LLVMExecutor<'p>, instr: &instruction::ExtractValue) -> Result<()> {
        debug!("{}", instr);
        let value = extract_value(&e.state, &instr.aggregate, &instr.indices)?;
        e.assign_result(instr, value)
    }

    /// Insert value into an aggregate value.
    ///
    /// The first operand should be either a struct or an array. The second operand is the first-
    /// class value to insert. The other operands are the indices that specify where the value
    /// should be inserted.
    fn insertvalue(e: &mut LLVMExecutor<'p>, instr: &instruction::InsertValue) -> Result<()> {
        debug!("{}", instr);
        let original = e.state.get_expr(&instr.aggregate)?;
        let value = e.state.get_expr(&instr.element)?;

        let (offset, _) = get_element_offset(&e.state, &instr.aggregate, &instr.indices)?;
        let value = original.replace_part(offset as u32, value);

        e.assign_result(instr, value)
    }

    // ---------------------------------------------------------------------------------------------
    // Memory access and Addressing Operations
    // ---------------------------------------------------------------------------------------------

    /// Allocate memory on the stack frame of the currently executing function. The memory is
    /// automatically cleaned up when the function returns.
    fn alloca(e: &mut LLVMExecutor<'p>, instr: &instruction::Alloca) -> Result<()> {
        debug!("{}", instr);
        let num_elements = e.state.get_expr(&instr.num_elements)?;
        let num_elements = num_elements.get_constant().unwrap();
        // let num_elements = instr.num_elements.to_value()?;
        let element_size = e.project.bit_size(&instr.allocated_type)? as u64;

        let mut allocation_size = element_size * num_elements;
        if allocation_size == 0 {
            warn!("zero sized alloca");
            allocation_size = e.project.ptr_size as u64;
        }

        let addr = e
            .state
            .memory
            .allocate(allocation_size, instr.alignment as u64)?;
        let addr = e.state.ctx.from_u64(addr, e.project.ptr_size);

        e.assign_result(instr, addr)
    }

    /// Load reads a value from memory.
    fn load(e: &mut LLVMExecutor<'p>, instr: &instruction::Load) -> Result<()> {
        debug!("{}", instr);
        let addr = e.state.get_expr(&instr.address)?;

        let target_ty = e.state.type_of(instr);
        let target_size = e.project.bit_size(&target_ty)?;
        let value = e.state.memory.read(&addr, target_size)?;
        let value = value.simplify();
        e.assign_result(instr, value)
    }

    /// Store writes to a value to memory.
    ///
    /// Accepts a value which will be written to the passed pointer address.
    fn store(e: &mut LLVMExecutor<'p>, instr: &instruction::Store) -> Result<()> {
        debug!("{}", instr);

        let value = e.state.get_expr(&instr.value)?;
        let addr = e.state.get_expr(&instr.address)?;
        e.state.memory.write(&addr, value)?;
        Ok(())
    }

    fn fence(_e: &mut LLVMExecutor<'p>, instr: &instruction::Fence) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    /// Atomically loads the value and compares it, if equal store a new value.
    fn cmpxchg(e: &mut LLVMExecutor<'p>, instr: &instruction::CmpXchg) -> Result<()> {
        debug!("{}", instr);
        // All operations are atomic, so just perform the operation.
        let addr = e.state.get_expr(&instr.address)?;
        let expected = e.state.get_expr(&instr.expected)?;
        let replacement = e.state.get_expr(&instr.replacement)?;

        let current = e.state.memory.read(&addr, expected.len())?;

        // Replace if the current value at the address it equal to the expected value.
        let condition = current._eq(&expected);
        let result = condition.ite(&replacement, &current);

        // Write the result to memory.
        e.state.memory.write(&addr, result.clone())?;

        // The instructions returns a struct of { expected type, condition i1 }.
        let return_value = condition.concat(&result);
        e.assign_result(instr, return_value)
    }

    /// Atomically modify memory.
    ///
    /// The contents of the address is atomically read, modified and written back. The original
    /// value is assigned to the resulting register.
    fn atomicrmw(e: &mut LLVMExecutor<'p>, instr: &instruction::AtomicRMW) -> Result<()> {
        debug!("{}", instr);
        let addr = e.state.get_expr(&instr.address)?;
        let rhs = e.state.get_expr(&instr.value)?;

        let lhs = e.state.memory.read(&addr, rhs.len())?;

        use instruction::RMWBinOp::*;
        let result = match instr.operation {
            Xchg => rhs,
            Add => lhs.add(&rhs),
            Sub => lhs.sub(&rhs),
            And => lhs.and(&rhs),
            Nand => lhs.and(&rhs).not(),
            Or => lhs.or(&rhs),
            Xor => lhs.xor(&rhs),
            Max => lhs.sgte(&rhs).ite(&lhs, &rhs),
            Min => lhs.slte(&rhs).ite(&lhs, &rhs),
            UMax => lhs.ugte(&rhs).ite(&lhs, &rhs),
            UMin => lhs.ulte(&rhs).ite(&lhs, &rhs),
            FAdd => todo!(),
            FSub => todo!(),
        };
        e.state.memory.write(&addr, result)?;

        e.assign_result(instr, lhs)
    }

    /// GetElementPtr calculates the offset into an array or struct from a base pointer.
    fn getelementptr(e: &mut LLVMExecutor<'p>, instr: &instruction::GetElementPtr) -> Result<()> {
        debug!("{}", instr);
        // TODO: Support vector of pointers. Ref: https://llvm.org/docs/LangRef.html#vector-of-pointers

        let target_address = gep(
            &e.state,
            &instr.address,
            instr.indices.iter().map(|op| op.into()),
            instr.in_bounds,
        )?;
        trace!("gep calculated address: {target_address:?}");

        e.assign_result(instr, target_address)
    }

    // ---------------------------------------------------------------------------------------------
    // Conversion Operations
    // ---------------------------------------------------------------------------------------------

    /// Truncate a value to the destination type size.
    ///
    /// *Requires* the source type to be *larger* than the destination type. Both types must be
    /// integers, or vectors of integers of the same length.
    fn trunc(e: &mut LLVMExecutor<'p>, instr: &instruction::Trunc) -> Result<()> {
        debug!("{}", instr);
        let symbol = convert_to_map(
            &e.state,
            &instr.to_type,
            &instr.operand,
            |symbol, target_size| symbol.slice(0, target_size - 1),
        )?;
        e.assign_result(instr, symbol)
    }

    /// Zero extends a value to the destination type size.
    ///
    /// *Requires* the source type to be *smaller* than the destination type. Both types must be
    /// integers, or vectors of integers of the same length.
    fn zext(e: &mut LLVMExecutor<'p>, instr: &instruction::ZExt) -> Result<()> {
        debug!("{}", instr);
        let symbol = convert_to_map(
            &e.state,
            &instr.to_type,
            &instr.operand,
            |symbol, target_size| symbol.zero_ext(target_size),
        )?;
        e.assign_result(instr, symbol)
    }

    /// Sign extend a value to the destination type size.
    ///
    /// *Requires* the source type to be *smaller* than the destination type. Both types must be
    /// integers, or vectors of integers of the same length.
    fn sext(e: &mut LLVMExecutor<'p>, instr: &instruction::SExt) -> Result<()> {
        debug!("{}", instr);
        let symbol = convert_to_map(
            &e.state,
            &instr.to_type,
            &instr.operand,
            |symbol, target_size| symbol.sign_ext(target_size),
        )?;
        e.assign_result(instr, symbol)
    }

    /// Convert a floating point value from a larger type to a smaller type.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn fptrunc(_: &mut LLVMExecutor<'p>, instr: &instruction::FPTrunc) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction(
            "fptrunc".to_owned(),
        ))
    }

    /// Convert a floating point value from a smaller type to a larger type.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn fpext(_: &mut LLVMExecutor<'p>, instr: &instruction::FPExt) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction(
            "fpext".to_owned(),
        ))
    }

    /// Convert a floating point to unsigned integer.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn fptoui(_: &mut LLVMExecutor<'p>, instr: &instruction::FPToUI) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction(
            "fptoui".to_owned(),
        ))
    }

    /// Convert floating point to signed integer.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn fptosi(_: &mut LLVMExecutor<'p>, instr: &instruction::FPToSI) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction(
            "fptosi".to_owned(),
        ))
    }

    /// Convert unsigned integer to floating point.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn uitofp(_: &mut LLVMExecutor<'p>, instr: &instruction::UIToFP) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction(
            "uitofp".to_owned(),
        ))
    }

    /// Convert signed integer to floating point.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn sitofp(_: &mut LLVMExecutor<'p>, instr: &instruction::SIToFP) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction(
            "sitofp".to_owned(),
        ))
    }

    /// Takes a pointer or a vector of pointers and converts to an integer or a vector of integers.
    ///
    /// The pointer is either truncated or zero extended if the sizes do not match. For vectors this
    /// happens element by element.
    fn ptrtoint(e: &mut LLVMExecutor<'p>, instr: &instruction::PtrToInt) -> Result<()> {
        debug!("{}", instr);
        let bv = convert_to_map(
            &e.state,
            &instr.to_type,
            &instr.operand,
            |symbol, target_size| symbol.resize_unsigned(target_size),
        )?;
        e.assign_result(instr, bv)
    }

    /// Takes an integer or a vector of integers and converts to a pointer or a vector of pointers.
    ///
    /// The integer is either truncated or zero extended if the sizes do not match. For vectors this
    /// happens element by element.
    fn inttoptr(e: &mut LLVMExecutor<'p>, instr: &instruction::IntToPtr) -> Result<()> {
        debug!("{}", instr);
        let bv = convert_to_map(
            &e.state,
            &instr.to_type,
            &instr.operand,
            |symbol, target_size| symbol.resize_unsigned(target_size),
        )?;
        e.assign_result(instr, bv)
    }

    /// Bitcast casts a value to to resulting type without changing any of the
    /// bits.
    ///
    /// Essentially a no-op, the implementation only stores the source value
    /// with the destination name and type.
    ///
    /// Reference: https://llvm.org/docs/LangRef.html#bitcast-to-instruction
    fn bitcast(e: &mut LLVMExecutor<'p>, instr: &instruction::BitCast) -> Result<()> {
        debug!("{}", instr);
        let bv = cast_to(&e.state, &instr.to_type, &instr.operand)?;
        e.assign_result(instr, bv)
    }

    fn addrspacecast(e: &mut LLVMExecutor<'p>, instr: &instruction::AddrSpaceCast) -> Result<()> {
        debug!("{}", instr);
        let bv = cast_to(&e.state, &instr.to_type, &instr.operand)?;
        e.assign_result(instr, bv)
    }

    // ---------------------------------------------------------------------------------------------
    // Other Operations
    // ---------------------------------------------------------------------------------------------

    fn icmp(e: &mut LLVMExecutor<'p>, instr: &instruction::ICmp) -> Result<()> {
        debug!("{}", instr);
        let result = icmp(&e.state, &instr.operand0, &instr.operand1, instr.predicate)?;
        e.assign_result(instr, result)
    }

    /// Compare floating point values.
    ///
    /// Floating point is currently unsupported. Always returns [LLVMExecutorError::UnsupportedInstruction].
    fn fcmp(_: &mut LLVMExecutor<'p>, instr: &instruction::FCmp) -> Result<()> {
        debug!("{}", instr);
        Err(LLVMExecutorError::UnsupportedInstruction("fcmp".to_owned()))
    }

    /// Phi selects one of the values based on which basic block was previously executed.
    fn phi(e: &mut LLVMExecutor<'p>, instr: &instruction::Phi) -> Result<()> {
        // Phi takes the value where block in [value, block] was the block that was *just* executed.
        debug!("{}", instr);

        let previous_block = e.state.stack_frames.last().unwrap().location.previous_block;
        if let Some(previous_block) = previous_block {
            let mut value = None;
            for (op, block_name) in instr.incoming_values.iter() {
                if &previous_block.name == block_name {
                    let symbol = e.state.get_expr(op)?;
                    value = Some(symbol);
                    break;
                }
            }

            match value {
                Some(value) => e.assign_result(instr, value),
                None => {
                    panic!("Could not find previous block in list")
                }
            }
        } else {
            panic!("No previous basic block");
        }
    }

    /// Select chooses one value based on a condition.
    ///
    /// If the condition is an `i1` it returns the first value if the condition is `1`, otherwise
    /// the second value.
    ///
    /// If the condition is a vector of `i1`s then element per element selection of the values
    /// are performed.
    fn select(e: &mut LLVMExecutor<'p>, instr: &instruction::Select) -> Result<()> {
        debug!("{}", instr);

        let condition = e.state.get_expr(&instr.condition)?;
        let true_value = e.state.get_expr(&instr.true_value)?;
        let false_value = e.state.get_expr(&instr.false_value)?;

        assert_eq!(true_value.len(), false_value.len());

        let ty = e.state.type_of(&instr.condition);
        let result = match ty.as_ref() {
            Type::IntegerType { bits: 1 } => condition.ite(&true_value, &false_value),
            Type::VectorType {
                element_type,
                num_elements,
                scalable: _,
            } => {
                let condition_ty = e.state.type_of(element_type);
                let condition_bits = e.project.bit_size(&condition_ty)?;
                assert_eq!(condition_bits, 1);

                // The values selected on should be vectors with same type an number of elements,
                // and should have the same number of elements as the condition.
                let lhs_ty = e.state.type_of(&instr.true_value);
                let rhs_ty = e.state.type_of(&instr.false_value);
                match (lhs_ty.as_ref(), rhs_ty.as_ref()) {
                    (
                        Type::VectorType {
                            element_type: ty0,
                            num_elements: num_elements0,
                            scalable: _,
                        },
                        Type::VectorType {
                            element_type: ty1,
                            num_elements: num_elements1,
                            scalable: _,
                        },
                    ) => {
                        assert_eq!(ty0, ty1);
                        assert_eq!(num_elements0, num_elements1);
                        assert_eq!(num_elements, num_elements0);

                        let num_elements = *num_elements as u32;
                        let inner_ty = e.state.type_of(ty0);
                        let size_in_bits = e.project.bit_size(&inner_ty)?;

                        (0..num_elements)
                            .map(|i| {
                                let low = i * size_in_bits;
                                let high = (i + 1) * size_in_bits - 1;
                                let true_value = true_value.slice(low, high);
                                let false_value = false_value.slice(low, high);

                                let condition = condition.slice(i, i);
                                condition.ite(&true_value, &false_value)
                            })
                            .reduce(|acc, v| v.concat(&acc))
                            .ok_or(LLVMExecutorError::MalformedInstruction)?
                    }
                    _ => return Err(LLVMExecutorError::MalformedInstruction),
                }
            }
            _ => panic!("select: expected either i1 or vector of i1s, got {ty:?}"),
        };

        e.assign_result(instr, result)
    }

    fn freeze(_: &mut LLVMExecutor<'p>, instr: &instruction::Freeze) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn call(e: &mut LLVMExecutor<'p>, instr: &instruction::Call) -> Result<()> {
        debug!("{}", instr);

        // let current_module = e.state.current_loc.module;
        let current_module = e.state.stack_frames.last().unwrap().location.module;
        let function_names = e.resolve_function(&instr.function)?;
        debug!("resolved functions: {:?}", function_names);
        if function_names.len() > 1 {
            panic!("Multiple symblic function pointers not supported yet");
        }
        let name = function_names.first().unwrap();
        let function = e.project.get_function(&name, current_module)?;

        let return_value = match function {
            FunctionType::Function { function, module } => {
                let arguments = instr
                    .arguments
                    .iter()
                    .map(|(op, _)| e.state.get_expr(op))
                    .collect::<Result<Vec<_>>>()?;

                e.call_fn(module, function, arguments)?
            }
            FunctionType::Hook(hook) => {
                let args: Vec<_> = instr.arguments.iter().map(|(op, _)| op).collect();
                hook(e, &args)?
            }
            FunctionType::Intrinsic(intrinsic) => {
                let args: Vec<_> = instr.arguments.iter().map(|(op, _)| op).collect();
                intrinsic(e, &args)?
            }
        };

        // Assign the return value if the call has a target.
        if let Some(name) = instr.dest.clone() {
            match return_value {
                ReturnValue::Value(symbol) => e.assign_register(name, symbol),

                // Expected return value but got void.
                ReturnValue::Void => Err(LLVMExecutorError::MalformedInstruction),
            }
        } else {
            // If it expected void that's fine. Note that the case where we returned a value
            // but the caller expected void is not covered.
            Ok(())
        }
    }

    fn va_arg(_: &mut LLVMExecutor<'p>, instr: &instruction::VAArg) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn landingpad(_: &mut LLVMExecutor<'p>, instr: &instruction::LandingPad) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn catchpad(_: &mut LLVMExecutor<'p>, instr: &instruction::CatchPad) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn cleanuppad(_: &mut LLVMExecutor<'p>, instr: &instruction::CleanupPad) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        llvm::ReturnValue,
        smt::{DContext, Expression},
        ExecutorError, Project, VM,
    };

    fn run(fn_name: &str) -> Vec<Result<Option<i64>, ExecutorError>> {
        let path = format!("./tests/unit_tests/instructions.bc");
        let project = Box::new(Project::from_path(&path).expect("Failed to created project"));
        let project = Box::leak(project);

        let context = Box::new(DContext::new());
        let context = Box::leak(context);
        let mut vm = VM::new(project, context, fn_name).expect("Failed to create VM");

        let mut path_results = Vec::new();
        while let Some((path_result, state)) = vm.run() {
            let path_result = match path_result {
                Ok(value) => match value {
                    ReturnValue::Value(value) => {
                        let value = state
                            .constraints
                            .get_value(&value)
                            .expect("Failed to get concrete value");
                        let binary_str = value.to_binary_string();
                        Ok(Some(u128::from_str_radix(&binary_str, 2).unwrap() as i64))
                    }
                    _ => Ok(None),
                },
                Err(e) => Err(e),
            };
            path_results.push(path_result);
        }

        println!("{path_results:x?}");
        path_results
    }

    #[test]
    fn test_add() {
        let res = run("test_add");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(15)));
    }

    // #[test]
    // fn test_add_overflow() {
    //     let res = run("test_add_overflow");
    //     assert_eq!(res.len(), 1);
    //     assert!(res[0].is_ok());
    //     assert_eq!(res[0], Ok(Some(15)));
    // }

    #[test]
    fn test_add_vector() {
        let res = run("test_add_vector");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0000_006E_0000_0019)));
    }

    #[test]
    fn test_sub() {
        let res = run("test_sub");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(10)));
    }

    #[test]
    fn test_mul() {
        let res = run("test_mul");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(35)));
    }

    #[test]
    fn test_udiv() {
        let res = run("test_udiv");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(20)));
    }

    #[test]
    fn test_sdiv() {
        let res = run("test_sdiv");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(-20)));
    }

    #[test]
    fn test_urem() {
        let res = run("test_urem");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(3)));
    }

    #[test]
    fn test_srem() {
        let res = run("test_srem");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(3)));
    }

    #[test]
    fn test_srem2() {
        let res = run("test_srem2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(-3)));
    }

    #[test]
    fn test_and() {
        let res = run("test_and");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(48)));
    }

    #[test]
    fn test_or() {
        let res = run("test_or");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(59)));
    }

    #[test]
    fn test_xor() {
        let res = run("test_xor");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(11)));
    }

    #[test]
    fn test_shl() {
        let res = run("test_shl");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(116)));
    }

    #[test]
    fn test_lshr() {
        let res = run("test_lshr");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(29)));
    }

    #[test]
    fn test_ashr() {
        let res = run("test_ashr");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(29)));
    }

    #[test]
    fn test_ashr2() {
        let res = run("test_ashr2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(221)));
    }

    #[test]
    fn test_extract_value_arr1() {
        let res = run("test_extract_value_arr1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(1)));
    }

    #[test]
    fn test_extract_value_arr2() {
        let res = run("test_extract_value_arr2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(2)));
    }

    #[test]
    fn test_extract_value_arr3() {
        let res = run("test_extract_value_arr3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(3)));
    }

    #[test]
    fn test_extract_value_arr4() {
        let res = run("test_extract_value_arr4");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(4)));
    }

    #[test]
    fn test_extract_value_struct1() {
        let res = run("test_extract_value_struct1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(4)));
    }

    #[test]
    fn test_extract_value_struct2() {
        let res = run("test_extract_value_struct2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(10)));
    }

    #[test]
    fn test_extract_value_struct3() {
        let res = run("test_extract_value_struct3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(1)));
    }

    #[test]
    fn test_extract_value_struct4() {
        let res = run("test_extract_value_struct4");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(2)));
    }

    #[test]
    fn test_insert_value_arr1() {
        let res = run("test_insert_value_arr1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0403020a)));
    }

    #[test]
    fn test_insert_value_arr2() {
        let res = run("test_insert_value_arr2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x04030a01)));
    }

    #[test]
    fn test_insert_value_arr3() {
        let res = run("test_insert_value_arr3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x040a0201)));
    }

    #[test]
    fn test_insert_value_arr4() {
        let res = run("test_insert_value_arr4");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0a030201)));
    }

    #[test]
    fn test_insert_value_arr5() {
        let res = run("test_insert_value_arr5");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0506070803040102)));
    }

    #[test]
    fn test_insert_value_struct1() {
        let res = run("test_insert_value_struct1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x00020001000a000f)));
    }

    #[test]
    fn test_insert_value_struct2() {
        let res = run("test_insert_value_struct2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x00020001000f0004)));
    }

    #[test]
    fn test_insert_value_struct3() {
        let res = run("test_insert_value_struct3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0002000f000a0004)));
    }

    #[test]
    fn test_insert_value_struct4() {
        let res = run("test_insert_value_struct4");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x000f0001000a0004)));
    }

    #[test]
    fn test_load_store1() {
        let res = run("test_load_store1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x3456)));
    }

    #[test]
    fn test_load_store2() {
        let res = run("test_load_store2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x56)));
    }

    #[test]
    fn test_gep1() {
        let res = run("test_gep1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(1)));
    }

    #[test]
    fn test_gep2() {
        let res = run("test_gep2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(2)));
    }

    #[test]
    fn test_gep3() {
        let res = run("test_gep3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(3)));
    }

    #[test]
    fn test_gep4() {
        let res = run("test_gep4");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(4)));
    }

    #[test]
    fn test_gep5() {
        let res = run("test_gep5");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(5)));
    }

    #[test]
    fn test_gep6() {
        let res = run("test_gep6");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(6)));
    }

    #[test]
    fn test_gep_arr() {
        let res = run("test_gep_arr");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(2)));
    }

    #[test]
    fn test_gep_arr2() {
        let res = run("test_gep_arr2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(2)));
    }

    #[test]
    fn test_bitcast1() {
        let res = run("test_bitcast1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x56781234)));
    }

    #[test]
    fn test_bitcast2() {
        let res = run("test_bitcast2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x78563412)));
    }

    #[test]
    fn test_trunc() {
        let res = run("test_trunc");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0xCD)));
    }

    #[test]
    fn test_zext() {
        let res = run("test_zext");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x00FF)));
    }

    #[test]
    fn test_zext_vec() {
        let res = run("test_zext_vec");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x000F00FF)));
    }

    #[test]
    fn test_sext() {
        let res = run("test_sext");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0xFFFF)));
    }

    #[test]
    fn test_inttoptr_trunc() {
        let res = run("test_inttoptr_trunc");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x5555666677778888)));
    }

    #[test]
    fn test_inttoptr_noop() {
        let res = run("test_inttoptr_noop");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1111222233334444)));
    }

    #[test]
    fn test_inttoptr_extend() {
        let res = run("test_inttoptr_extend");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0000000011112222)));
    }

    #[test]
    fn test_ptrtoint_trunc() {
        let res = run("test_ptrtoint_trunc");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x44)));
    }

    #[test]
    fn test_ptrtoint_noop() {
        let res = run("test_ptrtoint_noop");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1111222233334444)));
    }

    #[test]
    fn test_ptrtoint_vec_trunc() {
        let res = run("test_ptrtoint_vec_trunc");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x7777888833334444)));
    }

    #[test]
    fn test_addrspacecast() {
        let res = run("test_addrspacecast");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1111222233334444)));
    }

    #[test]
    fn test_icmp_eq_false() {
        let res = run("test_icmp_eq_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0)));
    }

    #[test]
    fn test_icmp_eq_true() {
        let res = run("test_icmp_eq_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1)));
    }

    #[test]
    fn test_icmp_ne_false() {
        let res = run("test_icmp_ne_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0)));
    }

    #[test]
    fn test_icmp_ne_true() {
        let res = run("test_icmp_ne_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1)));
    }

    #[test]
    fn test_icmp_ugt_false() {
        let res = run("test_icmp_ugt_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0)));
    }

    #[test]
    fn test_icmp_ugt_true() {
        let res = run("test_icmp_ugt_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1)));
    }

    #[test]
    fn test_icmp_uge_false() {
        let res = run("test_icmp_uge_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0)));
    }

    #[test]
    fn test_icmp_uge_true() {
        let res = run("test_icmp_uge_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1)));
    }

    #[test]
    fn test_icmp_ult_false() {
        let res = run("test_icmp_ult_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0)));
    }

    #[test]
    fn test_icmp_ult_true() {
        let res = run("test_icmp_ult_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1)));
    }

    #[test]
    fn test_icmp_ule_false() {
        let res = run("test_icmp_ule_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0)));
    }

    #[test]
    fn test_icmp_ule_true() {
        let res = run("test_icmp_ule_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1)));
    }

    #[test]
    fn test_icmp_sgt_false() {
        let res = run("test_icmp_sgt_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0)));
    }

    #[test]
    fn test_icmp_sgt_true() {
        let res = run("test_icmp_sgt_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1)));
    }

    #[test]
    fn test_icmp_sge_false() {
        let res = run("test_icmp_sge_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0)));
    }

    #[test]
    fn test_icmp_sge_true() {
        let res = run("test_icmp_sge_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1)));
    }

    #[test]
    fn test_icmp_slt_false() {
        let res = run("test_icmp_slt_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0)));
    }

    #[test]
    fn test_icmp_slt_true() {
        let res = run("test_icmp_slt_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1)));
    }

    #[test]
    fn test_icmp_sle_false() {
        let res = run("test_icmp_sle_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x0)));
    }

    #[test]
    fn test_icmp_sle_true() {
        let res = run("test_icmp_sle_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x1)));
    }

    #[test]
    fn test_icmp_eq_vec() {
        let res = run("test_icmp_eq_vec");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x5)));
    }

    #[test]
    fn test_phi1() {
        let res = run("test_phi1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0xab)));
    }

    #[test]
    fn test_phi2() {
        let res = run("test_phi2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0xcd)));
    }

    #[test]
    fn test_select1() {
        let res = run("test_select1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0xaa)));
    }

    #[test]
    fn test_select2() {
        let res = run("test_select2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0xbb)));
    }

    #[test]
    fn test_select_vec_values() {
        let res = run("test_select_vec_values");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x000000dd000000cc)));
    }

    #[test]
    fn test_select_vec_cond() {
        let res = run("test_select_vec_cond");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x000000dd000000aa)));
    }

    #[test]
    fn test_call() {
        let res = run("test_call");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0xabcd)));
    }

    #[test]
    fn test_vector_constant() {
        let res = run("test_vector_constant");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x5321)));
    }

    #[test]
    fn test_vector_constant2() {
        let res = run("test_vector_constant2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Ok(Some(0x04030201)));
    }
}

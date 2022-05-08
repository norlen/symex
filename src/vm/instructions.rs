use llvm_ir::{
    instruction::{self, BinaryOp, Instruction},
    terminator, Terminator, Type,
};
use log::{debug, trace, warn};

use crate::{
    common::{
        binop, cast_to, convert_to_map, extract_value, gep, get_element_offset, icmp, ToValue,
    },
    hooks::FnInfo,
    project::FunctionType,
    solver::BV,
    vm::{Call, Result, TerminatorResult, VMError, VM},
    ReturnValue,
};

impl<'a> VM<'a> {
    /// Process a single LLVM IR instruction.
    pub(super) fn process_instruction(&mut self, instr: &'a Instruction) -> Result<()> {
        match &instr {
            Instruction::Load(i) => self.load(i),
            Instruction::Store(i) => self.store(i),
            Instruction::Alloca(i) => self.alloca(i),
            Instruction::Add(i) => self.add(i),
            Instruction::Sub(i) => self.sub(i),
            Instruction::Mul(i) => self.mul(i),
            Instruction::UDiv(i) => self.udiv(i),
            Instruction::SDiv(i) => self.sdiv(i),
            Instruction::URem(i) => self.urem(i),
            Instruction::SRem(i) => self.srem(i),
            Instruction::And(i) => self.and(i),
            Instruction::Or(i) => self.or(i),
            Instruction::Xor(i) => self.xor(i),
            Instruction::Shl(i) => self.shl(i),
            Instruction::LShr(i) => self.lshr(i),
            Instruction::AShr(i) => self.ashr(i),
            Instruction::FAdd(i) => self.fadd(i),
            Instruction::FSub(i) => self.fsub(i),
            Instruction::FMul(i) => self.fmul(i),
            Instruction::FDiv(i) => self.fdiv(i),
            Instruction::FRem(i) => self.frem(i),
            Instruction::FNeg(i) => self.fneg(i),
            Instruction::ExtractElement(i) => self.extractelement(i),
            Instruction::InsertElement(i) => self.insertelement(i),
            Instruction::ShuffleVector(i) => self.shufflevector(i),
            Instruction::ExtractValue(i) => self.extractvalue(i),
            Instruction::InsertValue(i) => self.insertvalue(i),
            Instruction::Fence(i) => self.fence(i),
            Instruction::CmpXchg(i) => self.cmpxchg(i),
            Instruction::AtomicRMW(i) => self.atomicrmw(i),
            Instruction::GetElementPtr(i) => self.getelementptr(i),
            Instruction::Trunc(i) => self.trunc(i),
            Instruction::ZExt(i) => self.zext(i),
            Instruction::SExt(i) => self.sext(i),
            Instruction::FPTrunc(i) => self.fptrunc(i),
            Instruction::FPExt(i) => self.fpext(i),
            Instruction::FPToUI(i) => self.fptoui(i),
            Instruction::FPToSI(i) => self.fptosi(i),
            Instruction::UIToFP(i) => self.uitofp(i),
            Instruction::SIToFP(i) => self.sitofp(i),
            Instruction::PtrToInt(i) => self.ptrtoint(i),
            Instruction::IntToPtr(i) => self.inttoptr(i),
            Instruction::BitCast(i) => self.bitcast(i),
            Instruction::AddrSpaceCast(i) => self.addrspacecast(i),
            Instruction::ICmp(i) => self.icmp(i),
            Instruction::FCmp(i) => self.fcmp(i),
            Instruction::Phi(i) => self.phi(i),
            Instruction::Select(i) => self.select(i),
            Instruction::Freeze(i) => self.freeze(i),
            Instruction::Call(i) => self.call(i),
            Instruction::VAArg(i) => self.va_arg(i),
            Instruction::LandingPad(i) => self.landingpad(i),
            Instruction::CatchPad(i) => self.catchpad(i),
            Instruction::CleanupPad(i) => self.cleanuppad(i),
        }
    }

    /// Process a single LLVM IR terminator instruction.
    pub(super) fn process_terminator(
        &mut self,
        terminator: &'a Terminator,
    ) -> Result<TerminatorResult> {
        match terminator {
            Terminator::Ret(i) => self.ret(i),
            Terminator::Br(i) => self.br(i),
            Terminator::CondBr(i) => self.condbr(i),
            Terminator::Switch(i) => self.switch(i),
            Terminator::IndirectBr(i) => self.indirectbr(i),
            Terminator::Invoke(i) => self.invoke(i),
            Terminator::Resume(i) => self.resume(i),
            Terminator::Unreachable(i) => self.unreachable(i),
            Terminator::CleanupRet(i) => self.cleanupret(i),
            Terminator::CatchRet(i) => self.catchret(i),
            Terminator::CatchSwitch(i) => self.catchswitch(i),
            Terminator::CallBr(i) => self.callbr(i),
        }
    }

    // ---------------------------------------------------------------------------------------------
    // Terminator Instructions
    // ---------------------------------------------------------------------------------------------

    /// Returns control back to the caller, and optionally a return value.
    fn ret(&mut self, instr: &terminator::Ret) -> Result<TerminatorResult> {
        debug!("{}", instr);
        let value = if let Some(op) = &instr.return_operand {
            let value = self.state.get_var(op)?;
            Some(value)
        } else {
            None
        };

        // When returning the variable scope has to be destroyed.
        //
        // However, we may not reach this in the case of errors earlier, but that does not matter
        // since if an error is encountered the path is considered dead, and cannot be resumed.
        self.state.vars.leave_scope();

        Ok(TerminatorResult::Return(value))
    }

    /// Unconditionally jumps to a basic block inside the current function.
    fn br(&mut self, instr: &terminator::Br) -> Result<TerminatorResult> {
        debug!("{}", instr);
        self.branch(&instr.dest)
    }

    /// Jumps to another basic block inside the current function.
    ///
    /// This is the conditional version, so it can pick between two basic blocks. It will evaluate
    /// the condition and check if the condition can be `true`, `false`, or both. If more than one
    /// path can be selected a save point is created.
    ///
    /// If the condition cannot be either `true` or `false` [VMError::Unsat] is returned.
    fn condbr(&mut self, instr: &terminator::CondBr) -> Result<TerminatorResult> {
        debug!("{}", instr);

        let cond = self.state.get_var(&instr.condition)?;
        let true_possible = self.solver.is_sat_with_constraint(&cond)?;
        let false_possible = self.solver.is_sat_with_constraint(&cond.not())?;

        let target = match (true_possible, false_possible) {
            (true, true) => {
                // Explore `true` path, and save `false` path for later.
                self.save_backtracking_path(&instr.false_dest, Some(cond.not()))?;
                self.solver.assert(&cond);
                Ok(&instr.true_dest)
            }
            (true, false) => Ok(&instr.true_dest),
            (false, true) => Ok(&instr.false_dest),
            (false, false) => Err(VMError::Unsat),
        }?;

        self.branch(target)
    }

    /// Jump to any number of basic blocks inside the current function.
    ///
    /// A more general version compared to `condbr`. Multiple conditions with jump targets can be
    /// specified, with a default that is taken if none of the conditions apply.
    ///
    /// It will check which conditions are satisfiable, and create save points for these. It will
    /// then branch to the first `true` condition it found.
    fn switch(&mut self, instr: &terminator::Switch) -> Result<TerminatorResult> {
        debug!("{}", instr);
        let value = self.state.get_var(&instr.operand).unwrap();

        // The condition for the default term in the switch. The default case is built such that
        //   C = true ^ (val != path_cond_1) ^ (val != path_cond_2) ^ ...
        // So if the default one is the only path, we'll still explore.
        let mut default_cond = self.solver.bv_from_bool(true);

        let mut paths = Vec::new();

        // Check if any of the non-default cases can be reached.
        for (constant, target) in instr.dests.iter() {
            let path_cond = self.state.get_var(constant).unwrap();

            // Build default condition.
            default_cond = default_cond.and(&value.ne(&path_cond));

            let cond = value.eq(&path_cond);
            if self.solver.is_sat_with_constraint(&cond).unwrap() {
                debug!("switch: path {} possible", target);
                paths.push((target, cond));
            }
        }

        // Check if the default case can be reached.
        if self.solver.is_sat_with_constraint(&default_cond).unwrap() {
            debug!("switch: default path possible");
            paths.push((&instr.default_dest, default_cond));
        }

        // Save backtracking points for all paths except one.
        for (target, cond) in paths.iter().skip(1).cloned() {
            self.save_backtracking_path(target, Some(cond))?;
        }

        // Jump the the one path that didn't get saved as a backtracking point.
        if let Some((target, cond)) = paths.first() {
            self.solver.assert(cond);
            self.branch(target)
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
    fn indirectbr(&mut self, instr: &terminator::IndirectBr) -> Result<TerminatorResult> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("indirectbr".to_owned()))
    }

    /// Call the specified function with support for resuming at an exception label.
    ///
    /// If the function returns normally it resumes execution at the `normal` label, if not
    /// execution is resumed at the `exception` label.
    fn invoke(&mut self, instr: &'a terminator::Invoke) -> Result<TerminatorResult> {
        debug!("{}", instr);
        // Exception handling is not supported, to this is bascially the same as the regular call.
        //
        // When execptions are supported, these should be caught and the interpreter should then
        // instead jump to the exception label.

        let current_module = self.state.current_loc.module;
        let name = self.resolve_function(&instr.function)?;
        debug!("resolved function: {}", name);
        let function = self.project.get_function(&name, current_module)?;

        let return_value = match function {
            FunctionType::Hook(hook) => {
                let info = FnInfo::from_invoke(instr);
                hook(self, info)?
            }
            FunctionType::Function { function, module } => {
                let arguments = instr
                    .arguments
                    .iter()
                    .map(|(op, _)| self.state.get_var(op))
                    .collect::<Result<Vec<_>>>()?;

                self.call_fn(Call::Invoke(instr), module, function, arguments)?
            }
        };

        let name = instr.result.clone();
        match return_value {
            ReturnValue::Value(symbol) => self.state.assign_bv(name, symbol)?,
            ReturnValue::Void => {}
        }

        self.branch(&instr.return_label)
    }

    fn callbr(&mut self, instr: &terminator::CallBr) -> Result<TerminatorResult> {
        debug!("{}", instr);
        todo!()
    }

    fn resume(&mut self, instr: &terminator::Resume) -> Result<TerminatorResult> {
        debug!("{}", instr);
        todo!()
    }

    fn catchswitch(&mut self, instr: &terminator::CatchSwitch) -> Result<TerminatorResult> {
        debug!("{}", instr);
        todo!()
    }

    fn catchret(&mut self, instr: &terminator::CatchRet) -> Result<TerminatorResult> {
        debug!("{}", instr);
        todo!()
    }

    fn cleanupret(&mut self, instr: &terminator::CleanupRet) -> Result<TerminatorResult> {
        debug!("{}", instr);
        todo!()
    }

    /// Unreachable returns an [`VMError::UnreachableInstruction`] error.
    ///
    /// The `unreachable` instruction as it name says, should be unreachable. If this is called, it
    /// is most likely an error in the interpreter.
    fn unreachable(&mut self, instr: &terminator::Unreachable) -> Result<TerminatorResult> {
        debug!("{}", instr);
        Err(VMError::UnreachableInstruction)
    }

    // ---------------------------------------------------------------------------------------------
    // Unary Operations
    // ---------------------------------------------------------------------------------------------

    /// Negates the value of the operand.
    ///
    /// The value must be a floating point type, or a vector of floating points.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn fneg(&mut self, instr: &instruction::FNeg) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("fneg".to_owned()))
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
    fn add(&mut self, instr: &instruction::Add) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::add,
        )?;
        self.assign(instr, result)
    }

    /// Calculate the sum of two floating points or two vectors of floating points.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn fadd(&mut self, instr: &instruction::FAdd) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("fadd".to_owned()))
    }

    /// Calculate the difference of two integers or two vectors of integers.
    ///
    /// Both arguments must have the same types and must be integers or vectors of integers. On
    /// unsigned overflow the result is mod 2^n, where n is the size in bits.
    fn sub(&mut self, instr: &instruction::Sub) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::sub,
        )?;
        self.assign(instr, result)
    }

    /// Calculate the difference of two floating points or two vectors of floating points.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn fsub(&mut self, instr: &instruction::FSub) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("fsub".to_owned()))
    }

    /// Calculates the product of two integers or two vectors of integers.
    ///
    /// Both arguments must have the same types and must be integers or vectors of integers. On
    /// unsigned overflow the result is mod 2^n, where n is the size in bits.
    fn mul(&mut self, instr: &instruction::Mul) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::mul,
        )?;
        self.assign(instr, result)
    }

    /// Calculates the product of two floating points or two vectors of floating points.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn fmul(&mut self, instr: &instruction::FMul) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("fmul".to_owned()))
    }

    /// Calculate the quotient of two integers or two vectors of integers.
    ///
    /// Returns the unsigned quotient of the operands. The denominator cannot be zero.
    fn udiv(&mut self, instr: &instruction::UDiv) -> Result<()> {
        debug!("{}", instr);
        // TODO: We cannot divide by zero here, we may want to provide an analysis for checking that.
        // However, vectors are supported so this should not be done here. Instead it should be
        // done elsewhere I think? Or we could change it to provide a map, and then we can check
        // it ourselves.
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::udiv,
        )?;
        self.assign(instr, result)
    }

    /// Calculate the quotient of two integers or two vectors of integers.
    ///
    /// Returns the signed quotient of the operands. The denominator cannot be zero. Overflow here
    /// also leads to undefined behavior.
    fn sdiv(&mut self, instr: &instruction::SDiv) -> Result<()> {
        debug!("{}", instr);
        // TODO: Apart from div by zero (see above). The overflow could also be checked I guess,
        // example in docs is 32-bit div with -2147483648 by -1, this may be the only case. i.e.
        // INTx::MIN / -1
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::sdiv,
        )?;
        self.assign(instr, result)
    }

    /// Calculate the quotient of two floating points or two vectors of floating points.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn fdiv(&mut self, instr: &instruction::FDiv) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("fdiv".to_owned()))
    }

    /// Calculate the remainder from unsigned division of two integers or a vector of integers.
    ///
    /// The operation performs a division so the denominator cannot be zero.
    fn urem(&mut self, instr: &instruction::URem) -> Result<()> {
        debug!("{}", instr);
        // TODO: Check div by zero.
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::urem,
        )?;
        self.assign(instr, result)
    }

    /// Calculate the remainder from signed division of two integers or two vector of integers.
    ///
    /// The operation performs a division so the denominator cannot be zero.
    fn srem(&mut self, instr: &instruction::SRem) -> Result<()> {
        debug!("{}", instr);
        // TODO: Check div by zero.
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::srem,
        )?;
        self.assign(instr, result)
    }

    /// Calculate the remainder from division of two floating points or two vectors of floating
    /// points.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn frem(&mut self, instr: &instruction::FRem) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("frem".to_owned()))
    }

    // ---------------------------------------------------------------------------------------------
    // Bitwise Binary Operations
    // ---------------------------------------------------------------------------------------------

    /// Logical shift the first operand left by the number of bits specified by the second operand.
    ///
    /// Both parameters must be two integers or two vectors of integers. The second operand is
    /// treated as having unsigned ints.
    fn shl(&mut self, instr: &instruction::Shl) -> Result<()> {
        // TODO: There are a couple ways to get poison values. Read more about those.
        debug!("{}", instr);
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::sll,
        )?;
        self.assign(instr, result)
    }

    /// Logical shift left.
    fn lshr(&mut self, instr: &instruction::LShr) -> Result<()> {
        // TODO: There are a couple ways to get poison values. Read more about those.
        debug!("{}", instr);
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::srl,
        )?;
        self.assign(instr, result)
    }

    /// Arithmetic shift right.
    fn ashr(&mut self, instr: &instruction::AShr) -> Result<()> {
        // TODO: There are a couple ways to get poison values. Read more about those.
        debug!("{}", instr);
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::sra,
        )?;
        self.assign(instr, result)
    }

    /// Bitwise logical and.
    fn and(&mut self, instr: &instruction::And) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::and,
        )?;
        self.assign(instr, result)
    }

    /// Bitwise logical or.
    fn or(&mut self, instr: &instruction::Or) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::or,
        )?;
        self.assign(instr, result)
    }

    /// Bitwise logical xor.
    fn xor(&mut self, instr: &instruction::Xor) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BV::xor,
        )?;
        self.assign(instr, result)
    }

    // ---------------------------------------------------------------------------------------------
    // Vector Operations
    // ---------------------------------------------------------------------------------------------

    fn extractelement(&mut self, instr: &instruction::ExtractElement) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn insertelement(&mut self, instr: &instruction::InsertElement) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn shufflevector(&mut self, instr: &instruction::ShuffleVector) -> Result<()> {
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
    fn extractvalue(&mut self, instr: &instruction::ExtractValue) -> Result<()> {
        debug!("{}", instr);
        let value = extract_value(&self.state, &instr.aggregate, &instr.indices)?;
        self.assign(instr, value)
    }

    /// Insert value into an aggregate value.
    ///
    /// The first operand should be either a struct or an array. The second operand is the first-
    /// class value to insert. The other operands are the indices that specify where the value
    /// should be inserted.
    fn insertvalue(&mut self, instr: &instruction::InsertValue) -> Result<()> {
        debug!("{}", instr);
        let original = self.state.get_var(&instr.aggregate)?;
        let value = self.state.get_var(&instr.element)?;

        let (offset, _) = get_element_offset(&self.state, &instr.aggregate, &instr.indices)?;
        let value = original.replace_part(offset as u32, value);

        self.assign(instr, value)
    }

    // ---------------------------------------------------------------------------------------------
    // Memory access and Addressing Operations
    // ---------------------------------------------------------------------------------------------

    /// Allocate memory on the stack frame of the currently executing function. The memory is
    /// automatically cleaned up when the function returns.
    fn alloca(&mut self, instr: &instruction::Alloca) -> Result<()> {
        debug!("{}", instr);
        let num_elements = instr.num_elements.to_value()?;
        let element_size = self.project.bit_size(&instr.allocated_type)? as u64;

        let mut allocation_size = element_size * num_elements;
        if allocation_size == 0 {
            warn!("zero sized alloca");
            allocation_size = self.project.ptr_size as u64;
        }

        let addr = self
            .state
            .stack_alloc(allocation_size, instr.alignment as u64)?;

        self.assign(instr, addr)
    }

    /// Load reads a value from memory.
    fn load(&mut self, instr: &instruction::Load) -> Result<()> {
        debug!("{}", instr);
        let addr = self.state.get_var(&instr.address)?;

        let target_ty = self.state.type_of(instr);
        let target_size = self.project.bit_size(&target_ty)?;
        let value = self.state.mem.read(&addr, target_size)?;
        self.assign(instr, value)
    }

    /// Store writes to a value to memory.
    ///
    /// Accepts a value which will be written to the passed pointer address.
    fn store(&mut self, instr: &instruction::Store) -> Result<()> {
        debug!("{}", instr);

        let value = self.state.get_var(&instr.value)?;
        let addr = self.state.get_var(&instr.address)?;
        self.state.mem.write(&addr, value)?;
        Ok(())
    }

    fn fence(&mut self, instr: &instruction::Fence) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn cmpxchg(&mut self, instr: &instruction::CmpXchg) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn atomicrmw(&mut self, instr: &instruction::AtomicRMW) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    /// GetElementPtr calculates the offset into an array or struct from a base pointer.
    fn getelementptr(&mut self, instr: &instruction::GetElementPtr) -> Result<()> {
        debug!("{}", instr);
        // TODO: Support vector of pointers. Ref: https://llvm.org/docs/LangRef.html#vector-of-pointers

        let target_address = gep(
            &self.state,
            &instr.address,
            instr.indices.iter().map(|op| op.into()),
            instr.in_bounds,
        )?;
        trace!("gep calculated address: {target_address:?}");

        self.assign(instr, target_address)
    }

    // ---------------------------------------------------------------------------------------------
    // Conversion Operations
    // ---------------------------------------------------------------------------------------------

    /// Truncate a value to the destination type size.
    ///
    /// *Requires* the source type to be *larger* than the destination type. Both types must be
    /// integers, or vectors of integers of the same length.
    fn trunc(&mut self, instr: &instruction::Trunc) -> Result<()> {
        debug!("{}", instr);
        let symbol = convert_to_map(
            &self.state,
            &instr.to_type,
            &instr.operand,
            |symbol, target_size| symbol.slice(0, target_size - 1),
        )?;
        self.assign(instr, symbol)
    }

    /// Zero extends a value to the destination type size.
    ///
    /// *Requires* the source type to be *smaller* than the destination type. Both types must be
    /// integers, or vectors of integers of the same length.
    fn zext(&mut self, instr: &instruction::ZExt) -> Result<()> {
        debug!("{}", instr);
        let symbol = convert_to_map(
            &self.state,
            &instr.to_type,
            &instr.operand,
            |symbol, target_size| symbol.zero_ext(target_size),
        )?;
        self.assign(instr, symbol)
    }

    /// Sign extend a value to the destination type size.
    ///
    /// *Requires* the source type to be *smaller* than the destination type. Both types must be
    /// integers, or vectors of integers of the same length.
    fn sext(&mut self, instr: &instruction::SExt) -> Result<()> {
        debug!("{}", instr);
        let symbol = convert_to_map(
            &self.state,
            &instr.to_type,
            &instr.operand,
            |symbol, target_size| symbol.sign_ext(target_size),
        )?;
        self.assign(instr, symbol)
    }

    /// Convert a floating point value from a larger type to a smaller type.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn fptrunc(&mut self, instr: &instruction::FPTrunc) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("fptrunc".to_owned()))
    }

    /// Convert a floating point value from a smaller type to a larger type.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn fpext(&mut self, instr: &instruction::FPExt) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("fpext".to_owned()))
    }

    /// Convert a floating point to unsigned integer.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn fptoui(&mut self, instr: &instruction::FPToUI) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("fptoui".to_owned()))
    }

    /// Convert floating point to signed integer.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn fptosi(&mut self, instr: &instruction::FPToSI) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("fptosi".to_owned()))
    }

    /// Convert unsigned integer to floating point.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn uitofp(&mut self, instr: &instruction::UIToFP) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("uitofp".to_owned()))
    }

    /// Convert signed integer to floating point.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn sitofp(&mut self, instr: &instruction::SIToFP) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("sitofp".to_owned()))
    }

    /// Takes a pointer or a vector of pointers and converts to an integer or a vector of integers.
    ///
    /// The pointer is either truncated or zero extended if the sizes do not match. For vectors this
    /// happens element by element.
    fn ptrtoint(&mut self, instr: &instruction::PtrToInt) -> Result<()> {
        debug!("{}", instr);
        let bv = convert_to_map(
            &self.state,
            &instr.to_type,
            &instr.operand,
            |symbol, target_size| symbol.resize_unsigned(target_size),
        )?;
        self.assign(instr, bv)
    }

    /// Takes an integer or a vector of integers and converts to a pointer or a vector of pointers.
    ///
    /// The integer is either truncated or zero extended if the sizes do not match. For vectors this
    /// happens element by element.
    fn inttoptr(&mut self, instr: &instruction::IntToPtr) -> Result<()> {
        debug!("{}", instr);
        let bv = convert_to_map(
            &self.state,
            &instr.to_type,
            &instr.operand,
            |symbol, target_size| symbol.resize_unsigned(target_size),
        )?;
        self.assign(instr, bv)
    }

    /// Bitcast casts a value to to resulting type without changing any of the
    /// bits.
    ///
    /// Essentially a no-op, the implementation only stores the source value
    /// with the destination name and type.
    ///
    /// Reference: https://llvm.org/docs/LangRef.html#bitcast-to-instruction
    fn bitcast(&mut self, instr: &instruction::BitCast) -> Result<()> {
        debug!("{}", instr);
        let bv = cast_to(&self.state, &instr.to_type, &instr.operand)?;
        self.assign(instr, bv)
    }

    fn addrspacecast(&mut self, instr: &instruction::AddrSpaceCast) -> Result<()> {
        debug!("{}", instr);
        let bv = cast_to(&self.state, &instr.to_type, &instr.operand)?;
        self.assign(instr, bv)
    }

    // ---------------------------------------------------------------------------------------------
    // Other Operations
    // ---------------------------------------------------------------------------------------------

    fn icmp(&mut self, instr: &instruction::ICmp) -> Result<()> {
        debug!("{}", instr);
        let result = icmp(
            &self.state,
            &instr.operand0,
            &instr.operand1,
            instr.predicate,
        )?;
        self.assign(instr, result)
    }

    /// Compare floating point values.
    ///
    /// Floating point is currently unsupported. Always returns [VMError::UnsupportedInstruction].
    fn fcmp(&mut self, instr: &instruction::FCmp) -> Result<()> {
        debug!("{}", instr);
        Err(VMError::UnsupportedInstruction("fcmp".to_owned()))
    }

    /// Phi selects one of the values based on which basic block was previously executed.
    fn phi(&mut self, instr: &instruction::Phi) -> Result<()> {
        // Phi takes the value where block in [value, block] was the block that was *just* executed.
        debug!("{}", instr);

        if let Some(previous_block) = self.state.current_loc.previous_block {
            let mut value = None;
            for (op, block_name) in instr.incoming_values.iter() {
                if &previous_block.name == block_name {
                    let symbol = self.state.get_var(op)?;
                    value = Some(symbol);
                    break;
                }
            }

            match value {
                Some(value) => self.assign(instr, value),
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
    fn select(&mut self, instr: &instruction::Select) -> Result<()> {
        debug!("{}", instr);

        let condition = self.state.get_var(&instr.condition)?;
        let true_value = self.state.get_var(&instr.true_value)?;
        let false_value = self.state.get_var(&instr.false_value)?;

        assert_eq!(true_value.len(), false_value.len());

        let ty = self.state.type_of(&instr.condition);
        let result = match ty.as_ref() {
            Type::IntegerType { bits: 1 } => condition.ite(&true_value, &false_value),
            Type::VectorType {
                element_type,
                num_elements,
                scalable: _,
            } => {
                let condition_ty = self.state.type_of(element_type);
                let condition_bits = self.project.bit_size(&condition_ty)?;
                assert_eq!(condition_bits, 1);

                // The values selected on should be vectors with same type an number of elements,
                // and should have the same number of elements as the condition.
                let lhs_ty = self.state.type_of(&instr.true_value);
                let rhs_ty = self.state.type_of(&instr.false_value);
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
                        let inner_ty = self.state.type_of(ty0);
                        let size_in_bits = self.project.bit_size(&inner_ty)?;

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
                            .ok_or(VMError::MalformedInstruction)?
                    }
                    _ => return Err(VMError::MalformedInstruction),
                }
            }
            _ => panic!("select: expected either i1 or vector of i1s, got {ty:?}"),
        };

        self.assign(instr, result)
    }

    fn freeze(&mut self, instr: &instruction::Freeze) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn call(&mut self, instr: &'a instruction::Call) -> Result<()> {
        debug!("{}", instr);

        let current_module = self.state.current_loc.module;
        let name = self.resolve_function(&instr.function)?;
        debug!("resolved function: {}", name);
        let function = self.project.get_function(&name, current_module)?;

        let return_value = match function {
            FunctionType::Hook(hook) => {
                let info = FnInfo::from_call(instr);
                hook(self, info)?
            }
            FunctionType::Function { function, module } => {
                let arguments = instr
                    .arguments
                    .iter()
                    .map(|(op, _)| self.state.get_var(op))
                    .collect::<Result<Vec<_>>>()?;

                self.call_fn(Call::Call(instr), module, function, arguments)?
            }
        };

        // Assign the return value if the call has a target.
        if let Some(name) = instr.dest.clone() {
            match return_value {
                ReturnValue::Value(symbol) => self.state.assign_bv(name, symbol),

                // Expected return value but got void.
                ReturnValue::Void => Err(VMError::MalformedInstruction),
            }
        } else {
            // If it expected void that's fine. Note that the case where we returned a value
            // but the caller expected void is not covered.
            Ok(())
        }
    }

    fn va_arg(&mut self, instr: &instruction::VAArg) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn landingpad(&mut self, instr: &instruction::LandingPad) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn catchpad(&mut self, instr: &instruction::CatchPad) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn cleanuppad(&mut self, instr: &instruction::CleanupPad) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Project, ReturnValue, Solutions, VMError, VM};

    fn run(fn_name: &str) -> Vec<Result<Option<i64>, VMError>> {
        let path = format!("./tests/unit_tests/instructions.bc");
        let project = Project::from_path(&path).expect("Failed to created proejct");
        let mut vm = VM::new(fn_name, &project).expect("Failed to create VM");

        let mut path_results = Vec::new();
        while let Some(path_result) = vm.run() {
            let path_result = match path_result {
                Ok(value) => match value {
                    ReturnValue::Value(value) => {
                        let sol = vm.solver.get_solutions_for_bv(&value, 1).unwrap();
                        match sol {
                            Solutions::None => panic!("No solutions"),
                            Solutions::Exactly(s) => Ok(Some(s[0].as_u64().unwrap() as i64)),
                            Solutions::AtLeast(_) => panic!("More than one solution"),
                        }
                    }
                    ReturnValue::Void => Ok(None),
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

use llvm_ir::{
    instruction::{self, BasicBlock, Instruction, LLVMAtomicRMWBinOp, LLVMIntPredicate},
    Function, Type, Value,
};
use tracing::{debug, trace, warn};

use crate::{
    memory::to_bytes_u32,
    smt::{DContext, DExpr, SolverError},
    vm::{Overriden, StackFrame},
};

use super::{
    project::Project, state::LLVMState, vm::VM, AnalysisError, Hook, Intrinsic, LLVMExecutorError,
    Path, Result,
};

pub struct LLVMExecutor<'vm> {
    pub vm: &'vm mut VM,

    pub state: LLVMState,

    pub project: &'static Project,
}

pub enum PathResult {
    Success(Option<DExpr>),
    Failure(AnalysisError),
    Suppress,
}

pub struct CallFn {
    function: Value,
    arguments: Vec<Value>,
}

pub enum CallResult {
    CallFn(CallFn),
    Return(Option<DExpr>),
    AnalysisError(AnalysisError),
}

pub enum BlockResult {
    Branch(BasicBlock),
    CallFn(CallFn),
    Return(Option<DExpr>),
    AnalysisError(AnalysisError),
}

pub enum InstructionResult {
    Continue,
    Assign(DExpr),
    Branch(BasicBlock),
    CallFn(CallFn),
    Return(Option<DExpr>),
    AnalysisError(AnalysisError),
}

pub enum ResolvedFunction {
    Function(Function),
    Instrinic(Intrinsic),
    Hook(Hook),
}

impl<'vm> LLVMExecutor<'vm> {
    pub fn from_state(state: LLVMState, vm: &'vm mut VM, project: &'static Project) -> Self {
        Self { vm, state, project }
    }

    /// Resume execution from a stored path.
    ///
    /// When we restore the state from a stored path, the VM's call stack is empty. So it cannot
    /// use the VM's stack to know when to stop execution.
    ///
    /// `resume_execution` instead iteratively executes until all the callsites in the stored state
    /// have been exhausted.
    pub fn resume_execution(&mut self) -> Result<PathResult> {
        loop {
            let result = self.execute_function()?;

            match result {
                // Returned from the function. If we don't have any stack frames left, then we are
                // done. Otherwise, we continue execution from the previous stack frame.
                CallResult::Return(value) => {
                    if self.state.stack_frames.is_empty() {
                        return Ok(PathResult::Success(value));
                    }

                    // Assign return values from functions.
                    if let Some(result) = value {
                        let current_instruction =
                            self.state.current_frame()?.current_instruction().cloned().expect(
                                "Basic block should not be empty. Should have a terminator instruction",
                            );

                        let register = Value::Instruction(current_instruction);
                        self.assign_result(register, result)?;
                    }

                    // Resume execution on the next instruction.
                    self.state.current_frame_mut()?.increase_pc();
                }

                // We are calling another function. This will push a new stack frame and resume
                // execution from the first basic block in that function.
                CallResult::CallFn(call) => {
                    if self.state.stack_frames.len() >= 100 {
                        panic!("Call depth exceeded");
                    }

                    // The received function *may* be a symbolic address, and can reference multiple
                    // different functions. So we need to resolve all possible functions, and create
                    // new paths for all but one.
                    let function = self.resolve_function(call.function)?;
                    match function {
                        ResolvedFunction::Function(function) => {
                            // Create arguments to put on the new stack frame.
                            let arguments = call
                                .arguments
                                .into_iter()
                                .map(|arg| self.state.get_expr(&arg))
                                .collect::<Result<Vec<_>>>()?;

                            let stack_frame = StackFrame::new_from_function(function, &arguments)?;
                            self.state.stack_frames.push(stack_frame);
                        }
                        ResolvedFunction::Instrinic(_) | ResolvedFunction::Hook(_) => {
                            // For these we perform the entire function call at once, and handle the
                            // return. This is a bit of a special case.
                            let result = match function {
                                ResolvedFunction::Function(_) => unreachable!(),
                                ResolvedFunction::Instrinic(i) => i(self, &call.arguments),
                                ResolvedFunction::Hook(i) => i(self, &call.arguments),
                            }?;

                            let PathResult::Success(value) = result else {
                                return Ok(result);
                            };

                            if let Some(value) = value {
                                let current_instruction = self
                                    .state
                                    .current_frame()?
                                    .current_instruction()
                                    .cloned()
                                    .expect(
                                        "Basic block should not be empty. Should have a terminator instruction",
                                    );

                                let register = Value::Instruction(current_instruction);
                                self.assign_result(register, value)?;
                            }
                            self.state.current_frame_mut()?.increase_pc();
                        }
                    }
                }

                // End execution.
                CallResult::AnalysisError(error) => return Ok(PathResult::Failure(error)),
            }
        }
    }

    /// Execute a single function.
    ///
    /// This will iteratively go through each basic block until it hits a terminator that returns
    /// a value (or void).
    fn execute_function(&mut self) -> Result<CallResult> {
        loop {
            let result = self.execute_basic_block()?;
            match result {
                // Branch to another basic block in the same function.
                //
                // Keep the execution going until we leave the function.
                BlockResult::Branch(target) => {
                    self.state.current_frame_mut()?.set_basic_block(target)?
                }

                // Both of these will leave the current function, so return control to the caller.
                BlockResult::Return(value) => {
                    self.state.stack_frames.pop();
                    return Ok(CallResult::Return(value));
                }
                BlockResult::CallFn(call_fn) => {
                    return Ok(CallResult::CallFn(call_fn));
                }
                BlockResult::AnalysisError(error) => return Ok(CallResult::AnalysisError(error)),
            }
        }
    }

    /// Execute a single basic block.
    ///
    /// Resumes execution from the instruction stored in the current location and runs until it
    /// hits a terminator. This can either be a value, or a variant denoting a branch has occurred
    /// and that the callee should call this function again to resume execution in that basic block.
    fn execute_basic_block(&mut self) -> Result<BlockResult> {
        loop {
            let instruction = self
                .state
                .current_frame()?
                .current_instruction()
                .cloned()
                .expect("Basic block should not be empty. Should have a terminator instruction");

            let result = self.execute_instruction(&instruction)?;
            match result {
                // Continue execution in the same basic block.
                InstructionResult::Continue => {}

                // Assign the result of the instruction and continue execution.
                InstructionResult::Assign(result) => {
                    let value = Value::Instruction(instruction);
                    self.assign_result(value, result)?
                }

                // Return control to caller as we are leaving the current basic block.
                InstructionResult::Return(value) => {
                    return Ok(BlockResult::Return(value));
                }
                InstructionResult::Branch(branch) => return Ok(BlockResult::Branch(branch)),
                InstructionResult::CallFn(call_fn) => return Ok(BlockResult::CallFn(call_fn)),
                InstructionResult::AnalysisError(error) => {
                    return Ok(BlockResult::AnalysisError(error))
                }
            }

            self.state.current_frame_mut()?.increase_pc();
        }
    }

    fn assign_result(&mut self, value: Value, result: DExpr) -> Result<()> {
        self.state.current_frame_mut()?.set_register(value, result);
        Ok(())
    }

    /// Resolve a function address to a concrete function.
    fn resolve_function(&mut self, called_value: Value) -> Result<ResolvedFunction> {
        let fn_lookup = |function: Function| -> ResolvedFunction {
            if let Some(overriden) = self.project.get_function(function.name()) {
                match overriden {
                    Overriden::Intrinsic(i) => ResolvedFunction::Instrinic(i),
                    Overriden::Hook(h) => ResolvedFunction::Hook(h),
                }
            } else {
                ResolvedFunction::Function(function)
            }
        };

        // Fast path for non-address values.
        match called_value {
            Value::Function(function) => return Ok(fn_lookup(function)),
            Value::Metadata => todo!("Cannot call metadata"),
            Value::InlineAsm => todo!("Inline asm is not supported"),

            // TODO: Can we figure out a fast path for some of these as well?
            Value::Global(_) | Value::Instruction(_) | Value::Argument(_) | Value::Constant(_) => {}
        }

        let called_address = self.state.get_expr(&called_value)?;
        let called_address = match called_address.get_constant() {
            Some(c) => c,
            None => self
                .resolve_address(called_address)?
                .get_constant()
                .expect("Should be constant"),
        };

        let concrete_value = self
            .state
            .global_lookup_rev
            .get(&called_address)
            .cloned()
            .expect("Could not find global value to call at address: {called_address:x}");

        match concrete_value {
            Value::Function(function) => return Ok(fn_lookup(function)),
            Value::Global(_) => todo!(),
            Value::Instruction(_) => todo!(),
            Value::Argument(_) => todo!(),
            Value::Constant(_) => todo!(),
            Value::Metadata => todo!(),
            Value::InlineAsm => todo!(),
        }
    }

    /// Resolve an address expression to a single value.
    ///
    /// If the address contain more than one possible address, then we create new paths for all
    /// but one of the addresses.
    fn resolve_address(&mut self, address: DExpr) -> Result<DExpr> {
        if let Some(_) = address.get_constant() {
            return Ok(address);
        }

        // Create new paths for all but one of the addresses.
        let mut addresses = self.state.memory.resolve_addresses(&address, 50)?;
        for address in addresses.iter().skip(1) {
            let constraint = address._eq(&address);
            self.fork(constraint)?;
        }

        // If we received more than one possible address, then constrain our current address.
        if addresses.len() > 1 {
            let constraint = address._eq(&addresses[0]);
            self.state.constraints.assert(&constraint);
        }

        match addresses.is_empty() {
            true => panic!("no address..."),
            false => Ok(addresses.swap_remove(0)),
        }
    }

    pub fn fork(&mut self, constraint: DExpr) -> Result<()> {
        trace!("Save backtracking path: constraint={:?}", constraint);
        let forked_state = self.state.clone();
        let path = Path::new(forked_state, Some(constraint));

        self.vm.paths.save_path(path);
        Ok(())
    }

    fn fork_and_branch(&mut self, bb: BasicBlock, constraint: Option<DExpr>) -> Result<()> {
        trace!(
            "Save backtracking path: bb={:?}, constraint={:?}",
            bb,
            constraint
        );

        let mut state = self.state.clone();
        state.current_frame_mut()?.set_basic_block(bb)?;

        let path = Path::new(state, constraint);
        self.vm.paths.save_path(path);
        Ok(())
    }

    fn execute_instruction(&mut self, i: &Instruction) -> Result<InstructionResult> {
        match i {
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
            Instruction::ExtractElement(i) => self.extract_element(i),
            Instruction::InsertElement(i) => self.insert_element(i),
            Instruction::ShuffleVector(i) => self.shuffle_vector(i),
            Instruction::ExtractValue(i) => self.extract_value(i),
            Instruction::InsertValue(i) => self.insert_value(i),
            Instruction::Fence(i) => self.fence(i),
            Instruction::CmpXchg(i) => self.cmp_xchg(i),
            Instruction::AtomicRMW(i) => self.atomic_rmw(i),
            Instruction::GetElementPtr(i) => self.get_element_ptr(i),
            Instruction::Trunc(i) => self.trunc(i),
            Instruction::ZExt(i) => self.zext(i),
            Instruction::SExt(i) => self.sext(i),
            Instruction::FPTrunc(i) => self.fp_trunc(i),
            Instruction::FPExt(i) => self.fp_ext(i),
            Instruction::FPToUI(i) => self.fp_to_ui(i),
            Instruction::FPToSI(i) => self.fp_to_si(i),
            Instruction::UIToFP(i) => self.ui_to_fp(i),
            Instruction::SIToFP(i) => self.si_to_fp(i),
            Instruction::PtrToInt(i) => self.ptr_to_int(i),
            Instruction::IntToPtr(i) => self.int_to_ptr(i),
            Instruction::BitCast(i) => self.bit_cast(i),
            Instruction::AddrSpaceCast(i) => self.address_space_cast(i),
            Instruction::ICmp(i) => self.icmp(i),
            Instruction::FCmp(i) => self.fcmp(i),
            Instruction::Phi(i) => self.phi(i),
            Instruction::Select(i) => self.select(i),
            Instruction::Freeze(i) => self.freeze(i),
            Instruction::Call(i) => self.call(i),
            Instruction::VAArg(i) => self.va_arg(i),
            Instruction::LandingPad(i) => self.landing_pad(i),
            Instruction::CatchPad(i) => self.catch_pad(i),
            Instruction::CleanupPad(i) => self.cleanup_pad(i),
            Instruction::Ret(i) => self.ret(i),
            Instruction::Br(i) => self.br(i),
            Instruction::CondBr(i) => self.cond_br(i),
            Instruction::Switch(i) => self.switch(i),
            Instruction::IndirectBr(i) => self.indirect_br(i),
            Instruction::Invoke(i) => self.invoke(i),
            Instruction::Resume(i) => self.resume(i),
            Instruction::Unreachable(i) => self.unreachable(i),
            Instruction::CleanupRet(i) => self.cleanup_ret(i),
            Instruction::CatchRet(i) => self.catch_ret(i),
            Instruction::CatchSwitch(i) => self.catch_switch(i),
            Instruction::CallBr(i) => self.call_br(i),
        }
    }

    /// Load reads a value from memory.
    fn load(&mut self, i: &instruction::Load) -> Result<InstructionResult> {
        debug!("{i}");
        let address = self.state.get_expr(&i.address())?;
        let address = self.resolve_address(address)?;

        let size = bit_size(&i.result_type(), self.project.ptr_size).unwrap();
        let value = self.state.memory.read(&address, size)?;
        Ok(InstructionResult::Assign(value))
    }

    /// Write [`DExpr`] to memory.
    fn store(&mut self, i: &instruction::Store) -> Result<InstructionResult> {
        debug!("{i}");
        let value = self.state.get_expr(&i.value())?;
        let address = self.state.get_expr(&i.address())?;
        let address = self.resolve_address(address)?;

        self.state.memory.write(&address, value)?;
        Ok(InstructionResult::Continue)
    }

    fn alloca(&mut self, i: &instruction::Alloca) -> Result<InstructionResult> {
        debug!("{i}");
        let num_elements = i.num_elements();
        let num_elements = match self.state.get_expr(&num_elements)?.get_constant() {
            Some(c) => c,
            None => todo!("Handle non-constant alloca"),
        };

        let allocated_type = i.allocated_type();
        let allocated_size = bit_size(&allocated_type, self.project.ptr_size).unwrap() as u64;
        let allocated_size = match allocated_size * num_elements {
            0 => {
                warn!("Zero-sized alloca");
                // Just allocate some memory for this, not really sure how to handle this case.
                self.project.ptr_size as u64
            }
            n => n,
        };

        let alignment = i.alignment() as u64;
        let address = self.state.memory.allocate(allocated_size, alignment)?;
        let address = self.state.ctx.from_u64(address, self.project.ptr_size);

        Ok(InstructionResult::Assign(address))
    }

    fn add(&mut self, i: &instruction::Add) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::add)?;
        Ok(InstructionResult::Assign(result))
    }

    fn sub(&mut self, i: &instruction::Sub) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::sub)?;
        Ok(InstructionResult::Assign(result))
    }

    fn mul(&mut self, i: &instruction::Mul) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::mul)?;
        Ok(InstructionResult::Assign(result))
    }

    fn udiv(&mut self, i: &instruction::UDiv) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::udiv)?;
        Ok(InstructionResult::Assign(result))
    }

    fn sdiv(&mut self, i: &instruction::SDiv) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::sdiv)?;
        Ok(InstructionResult::Assign(result))
    }

    fn urem(&mut self, i: &instruction::URem) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::urem)?;
        Ok(InstructionResult::Assign(result))
    }

    fn srem(&mut self, i: &instruction::SRem) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::srem)?;
        Ok(InstructionResult::Assign(result))
    }

    fn and(&mut self, i: &instruction::And) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::and)?;
        Ok(InstructionResult::Assign(result))
    }

    fn or(&mut self, i: &instruction::Or) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::or)?;
        Ok(InstructionResult::Assign(result))
    }

    fn xor(&mut self, i: &instruction::Xor) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::xor)?;
        Ok(InstructionResult::Assign(result))
    }

    fn shl(&mut self, i: &instruction::Shl) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::sll)?;
        Ok(InstructionResult::Assign(result))
    }

    fn lshr(&mut self, i: &instruction::LShr) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::srl)?;
        Ok(InstructionResult::Assign(result))
    }

    fn ashr(&mut self, i: &instruction::AShr) -> Result<InstructionResult> {
        debug!("{i}");
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), DExpr::sra)?;
        Ok(InstructionResult::Assign(result))
    }

    fn fadd(&mut self, _i: &instruction::FAdd) -> Result<InstructionResult> {
        Err(LLVMExecutorError::UnsupportedInstruction("fadd".to_owned()))
    }

    fn fsub(&mut self, _i: &instruction::FSub) -> Result<InstructionResult> {
        Err(LLVMExecutorError::UnsupportedInstruction("fsub".to_owned()))
    }

    fn fmul(&mut self, _i: &instruction::FMul) -> Result<InstructionResult> {
        Err(LLVMExecutorError::UnsupportedInstruction("fmul".to_owned()))
    }

    fn fdiv(&mut self, _i: &instruction::FDiv) -> Result<InstructionResult> {
        Err(LLVMExecutorError::UnsupportedInstruction("fdiv".to_owned()))
    }

    fn frem(&mut self, _i: &instruction::FRem) -> Result<InstructionResult> {
        Err(LLVMExecutorError::UnsupportedInstruction("frem".to_owned()))
    }

    fn fneg(&mut self, _i: &instruction::FNeg) -> Result<InstructionResult> {
        Err(LLVMExecutorError::UnsupportedInstruction("fneg".to_owned()))
    }

    fn extract_element(&mut self, _i: &instruction::ExtractElement) -> Result<InstructionResult> {
        todo!()
    }

    fn insert_element(&mut self, _i: &instruction::InsertElement) -> Result<InstructionResult> {
        todo!()
    }

    fn shuffle_vector(&mut self, _i: &instruction::ShuffleVector) -> Result<InstructionResult> {
        todo!()
    }

    fn extract_value(&mut self, i: &instruction::ExtractValue) -> Result<InstructionResult> {
        debug!("{i}");
        let aggregate = i.aggregate();
        let (lower, upper) = get_element_offset(&self.state, aggregate.ty(), i.indices())?;

        let aggregate = self.state.get_expr(&aggregate)?;
        assert!(aggregate.len() >= upper && upper > 0);

        let extracted = aggregate.slice(lower, upper - 1);
        Ok(InstructionResult::Assign(extracted))
    }

    fn insert_value(&mut self, i: &instruction::InsertValue) -> Result<InstructionResult> {
        debug!("{i}");
        let aggregate = i.aggregate();
        let (lower, upper) = get_element_offset(&self.state, aggregate.ty(), i.indices())?;

        let element = self.state.get_expr(&i.element())?;
        assert_eq!(upper - lower, element.len());

        let aggregate = self.state.get_expr(&aggregate)?;
        let new_aggregate = aggregate.replace_part(lower, element);
        Ok(InstructionResult::Assign(new_aggregate))
    }

    fn fence(&mut self, _i: &instruction::Fence) -> Result<InstructionResult> {
        todo!()
    }

    fn cmp_xchg(&mut self, i: &instruction::CmpXchg) -> Result<InstructionResult> {
        debug!("{i}");
        let address = self.state.get_expr(&i.address())?;
        let address = self.resolve_address(address)?;

        let cmp = self.state.get_expr(&i.cmp())?;
        let new_value = self.state.get_expr(&i.new_value())?;

        // Replace the old value with the new value if the old value matches the comparison value.
        let old_value = self.state.memory.read(&address, new_value.len())?;
        let condition = old_value._eq(&cmp);
        let result = condition.ite(&new_value, &old_value);
        self.state.memory.write(&address, result.clone())?;

        // The instructions returns a struct of { result, condition i1 }.
        let result = condition.concat(&result); // TODO: Is this correct?
        Ok(InstructionResult::Assign(result))
    }

    /// Atomically modify memory.
    ///
    /// The contents of the address is atomically read, modified and written back. The original
    /// value is assigned to the resulting register.
    fn atomic_rmw(&mut self, i: &instruction::AtomicRMW) -> Result<InstructionResult> {
        debug!("{i}");
        let address = self.state.get_expr(&i.address())?;
        let address = self.resolve_address(address)?;

        let new_value = self.state.get_expr(&i.value())?;
        let old_value = self.state.memory.read(&address, new_value.len())?;

        let result = match i.operation() {
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpXchg => new_value,
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpAdd => old_value.add(&new_value),
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpSub => old_value.sub(&new_value),
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpAnd => old_value.and(&new_value),
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpNand => old_value.and(&new_value).not(),
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpOr => old_value.or(&new_value),
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpXor => old_value.xor(&new_value),
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpMax => {
                old_value.sgte(&new_value).ite(&old_value, &new_value)
            }
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpMin => {
                old_value.slte(&new_value).ite(&old_value, &new_value)
            }
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpUMax => {
                old_value.ugte(&new_value).ite(&old_value, &new_value)
            }
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpUMin => {
                old_value.ulte(&new_value).ite(&old_value, &new_value)
            }
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpFAdd => todo!(),
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpFSub => todo!(),
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpFMax => todo!(),
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpFMin => todo!(),
        };
        self.state.memory.write(&address, result)?;
        Ok(InstructionResult::Assign(old_value))
    }

    fn get_element_ptr(&mut self, i: &instruction::GetElementPtr) -> Result<InstructionResult> {
        debug!("{i}");
        let ptr_size = self.state.project.ptr_size;

        // The `in_bounds` field is pretty useless for figuring out if the address is actually within
        // the type. We cannot use any type information here (https://llvm.org/docs/GetElementPtr.html)
        //
        // So we have to get the actual underlying allocation for this, but as the address is symbolic
        // that poses a problem.
        let address = i.address();
        if !address.ty().is_pointer() {
            panic!("getelementptr address should always be a pointer")
        }
        let address = self.state.get_expr(&address)?;
        let address = self.resolve_address(address)?;

        let source_element_type = i.source_element_type();
        let indices = i.indices();

        // The first argument is *always* a pointer, so the first index always determine
        // "which element" we access, e.g., if we get an array.
        let mut address = {
            let Some(index) = indices.first() else {
                panic!("getelementptr should always have at least one index");
            };

            let index = self.state.get_expr(index)?;
            let index = index.zero_ext(ptr_size).simplify();

            let size_bytes = byte_size(&source_element_type, ptr_size)?;
            let size_bytes = self.state.ctx.from_u64(size_bytes.into(), ptr_size);
            let offset = index.mul(&size_bytes);
            address.add(&offset)
        };

        let mut curr_ty = source_element_type;
        for index in indices.iter().skip(1) {
            let index = self.state.get_expr(index)?;
            let index = index.zero_ext(ptr_size).simplify();
            let (offset, ty) = byte_offset(&curr_ty, &index, ptr_size, &self.state.ctx)?;
            println!("index: {index:?}, offset: {offset:?}");

            address = address.add(&offset);
            curr_ty = ty;
        }

        Ok(InstructionResult::Assign(address))
    }

    fn trunc(&mut self, i: &instruction::Trunc) -> Result<InstructionResult> {
        debug!("{i}");
        let f = |value: DExpr, target_size: u32| value.slice(0, target_size - 1);
        let result = convert_to_map(&mut self.state, i.value(), &i.to_type(), f)?;
        Ok(InstructionResult::Assign(result))
    }

    fn zext(&mut self, i: &instruction::ZExt) -> Result<InstructionResult> {
        debug!("{i}");
        let f = |value: DExpr, target_size: u32| value.zero_ext(target_size);
        let result = convert_to_map(&mut self.state, i.value(), &i.to_type(), f)?;
        Ok(InstructionResult::Assign(result))
    }

    fn sext(&mut self, i: &instruction::SExt) -> Result<InstructionResult> {
        debug!("{i}");
        let f = |value: DExpr, target_size: u32| value.sign_ext(target_size);
        let result = convert_to_map(&mut self.state, i.value(), &i.to_type(), f)?;
        Ok(InstructionResult::Assign(result))
    }

    fn fp_trunc(&mut self, _i: &instruction::FPTrunc) -> Result<InstructionResult> {
        todo!()
    }

    fn fp_ext(&mut self, _i: &instruction::FPExt) -> Result<InstructionResult> {
        todo!()
    }

    fn fp_to_ui(&mut self, _i: &instruction::FPToUI) -> Result<InstructionResult> {
        todo!()
    }

    fn fp_to_si(&mut self, _i: &instruction::FPToSI) -> Result<InstructionResult> {
        todo!()
    }

    fn ui_to_fp(&mut self, _i: &instruction::UIToFP) -> Result<InstructionResult> {
        todo!()
    }

    fn si_to_fp(&mut self, _i: &instruction::SIToFP) -> Result<InstructionResult> {
        todo!()
    }

    fn ptr_to_int(&mut self, i: &instruction::PtrToInt) -> Result<InstructionResult> {
        debug!("{i}");
        let f = |value: DExpr, target_size: u32| value.resize_unsigned(target_size);
        let result = convert_to_map(&mut self.state, i.value(), &i.to_type(), f)?;
        Ok(InstructionResult::Assign(result))
    }

    fn int_to_ptr(&mut self, i: &instruction::IntToPtr) -> Result<InstructionResult> {
        debug!("{i}");
        let f = |value: DExpr, target_size: u32| value.resize_unsigned(target_size);
        let result = convert_to_map(&mut self.state, i.value(), &i.to_type(), f)?;
        Ok(InstructionResult::Assign(result))
    }

    fn bit_cast(&mut self, i: &instruction::BitCast) -> Result<InstructionResult> {
        debug!("{i}");
        let value = self.state.get_expr(&i.value())?;
        Ok(InstructionResult::Assign(value))
    }

    fn address_space_cast(&mut self, i: &instruction::AddrSpaceCast) -> Result<InstructionResult> {
        debug!("{i}");
        let value = self.state.get_expr(&i.value())?;
        Ok(InstructionResult::Assign(value))
    }

    fn icmp(&mut self, i: &instruction::ICmp) -> Result<InstructionResult> {
        debug!("{i}");
        let f = |lhs: &DExpr, rhs: &DExpr| match i.predicate() {
            LLVMIntPredicate::LLVMIntEQ => lhs._eq(&rhs),
            LLVMIntPredicate::LLVMIntNE => lhs._ne(&rhs),
            LLVMIntPredicate::LLVMIntUGT => lhs.ugt(&rhs),
            LLVMIntPredicate::LLVMIntUGE => lhs.ugte(&rhs),
            LLVMIntPredicate::LLVMIntULT => lhs.ult(&rhs),
            LLVMIntPredicate::LLVMIntULE => lhs.ulte(&rhs),
            LLVMIntPredicate::LLVMIntSGT => lhs.sgt(&rhs),
            LLVMIntPredicate::LLVMIntSGE => lhs.sgte(&rhs),
            LLVMIntPredicate::LLVMIntSLT => lhs.slt(&rhs),
            LLVMIntPredicate::LLVMIntSLE => lhs.slte(&rhs),
        };
        let result = binop(&mut self.state, &i.lhs(), &i.rhs(), f)?;
        Ok(InstructionResult::Assign(result))
    }

    fn fcmp(&mut self, _i: &instruction::FCmp) -> Result<InstructionResult> {
        todo!()
    }

    fn phi(&mut self, i: &instruction::Phi) -> Result<InstructionResult> {
        debug!("{i}");

        let Some(previous_block) = self.state.current_frame()?.previous_block() else {
            panic!("Phi instruction without previous block");
        };

        for (block, value) in i.incoming() {
            if &block != previous_block {
                continue;
            }

            let value = self.state.get_expr(&value)?;
            return Ok(InstructionResult::Assign(value));
        }
        panic!("Phi instruction without matching incoming value for previous block");
    }

    fn select(&mut self, i: &instruction::Select) -> Result<InstructionResult> {
        debug!("{i}");
        let condition = i.condition();
        let true_value = i.true_value();
        let false_value = i.false_value();

        let condition_expr = self.state.get_expr(&condition)?;
        let true_expr = self.state.get_expr(&true_value)?;
        let false_expr = self.state.get_expr(&false_value)?;
        assert_eq!(true_expr.len(), false_expr.len());

        let result = match condition.ty() {
            Type::Integer(t) if t.bits() == 1 => condition_expr.ite(&true_expr, &false_expr),
            Type::Vector(t) => {
                let condition_element_ty = t.element_type();
                if !matches!(&condition_element_ty, Type::Integer(t) if t.bits() == 1) {
                    panic!("Select condition vector must be of i1 type")
                }

                let size = match (true_value.ty(), false_value.ty()) {
                    (Type::Vector(lhs), Type::Vector(rhs)) => {
                        assert_eq!(lhs.num_elements(), rhs.num_elements());
                        assert_eq!(lhs.element_type(), rhs.element_type());
                        bit_size(&lhs.element_type(), self.state.project.ptr_size)?
                    }
                    _ => panic!("Select true and false value must be vectors"),
                };
                let num_elements = t.num_elements();

                (0..num_elements)
                    .map(|i| {
                        let low = i * size;
                        let high = (i + 1) * size - 1;

                        let condition = condition_expr.slice(i, i);
                        condition.ite(&true_expr.slice(low, high), &false_expr.slice(low, high))
                    })
                    .reduce(|acc, v| v.concat(&acc))
                    .ok_or(LLVMExecutorError::MalformedInstruction)?
            }
            _ => return Err(LLVMExecutorError::MalformedInstruction),
        };
        Ok(InstructionResult::Assign(result))
    }

    fn freeze(&mut self, _i: &instruction::Freeze) -> Result<InstructionResult> {
        todo!()
    }

    fn call(&mut self, i: &instruction::Call) -> Result<InstructionResult> {
        debug!("{i}");
        let call_fn = CallFn {
            function: i.called_value(),
            arguments: i.arguments(),
        };
        Ok(InstructionResult::CallFn(call_fn))
    }

    fn va_arg(&mut self, _i: &instruction::VAArg) -> Result<InstructionResult> {
        todo!()
    }

    fn landing_pad(&mut self, _i: &instruction::LandingPad) -> Result<InstructionResult> {
        todo!()
    }

    fn catch_pad(&mut self, _i: &instruction::CatchPad) -> Result<InstructionResult> {
        todo!()
    }

    fn cleanup_pad(&mut self, _i: &instruction::CleanupPad) -> Result<InstructionResult> {
        todo!()
    }

    fn ret(&mut self, i: &instruction::Ret) -> Result<InstructionResult> {
        debug!("{i}");
        let value = if let Some(value) = i.return_value() {
            Some(self.state.get_expr(&value)?)
        } else {
            None
        };
        Ok(InstructionResult::Return(value))
    }

    fn br(&mut self, i: &instruction::Br) -> Result<InstructionResult> {
        debug!("{i}");
        Ok(InstructionResult::Branch(i.destination()))
    }

    fn cond_br(&mut self, i: &instruction::CondBr) -> Result<InstructionResult> {
        debug!("{i}");
        let c = self.state.get_expr(&i.condition())?.simplify();

        // Fast path: skip checking satisfiability if the condition is constant.
        if let Some(target) = c.get_constant_bool() {
            let target = if target {
                i.true_destination()
            } else {
                i.false_destination()
            };
            return Ok(InstructionResult::Branch(target));
        }

        let true_possible = self.state.constraints.is_sat_with_constraint(&c)?;
        let false_possible = self.state.constraints.is_sat_with_constraint(&c.not())?;
        let target = match (true_possible, false_possible) {
            (true, true) => {
                // Explore `true` path, and save `false` path for later.
                self.fork_and_branch(i.false_destination(), Some(c.not()))?;

                self.state.constraints.assert(&c);
                Ok(i.true_destination())
            }
            (true, false) => Ok(i.true_destination()),
            (false, true) => Ok(i.false_destination()),
            (false, false) => Err(SolverError::Unsat),
        }?;
        Ok(InstructionResult::Branch(target))
    }

    fn switch(&mut self, i: &instruction::Switch) -> Result<InstructionResult> {
        debug!("{i}");
        let condition = self.state.get_expr(&i.condition())?.simplify();

        // The condition for the default term in the switch. The default case is built such that
        //   C = true ^ (val != path_cond_1) ^ (val != path_cond_2) ^ ...
        // So if the default one is the only path, we'll still explore.
        let mut default_cond = self.state.ctx.from_bool(true);

        let mut possible_paths = Vec::new();

        // Check if any of the non-default cases can be reached.
        for (value, bb) in i.cases() {
            let path_condition = self.state.get_expr(&value).unwrap();

            // Build default condition.
            default_cond = default_cond.and(&condition._ne(&path_condition));

            let constraint = condition._eq(&path_condition);
            if self.state.constraints.is_sat_with_constraint(&constraint)? {
                debug!("{i}: path {:?} possible", bb);
                possible_paths.push((bb, constraint));
            }
        }

        let can_reach_default = self
            .state
            .constraints
            .is_sat_with_constraint(&default_cond)?;

        if can_reach_default {
            debug!("{i}: default path possible");
            possible_paths.push((i.default_destination(), default_cond));
        }

        let Some((target, constraint)) = possible_paths.pop() else {
            panic!("Switch instruction without any possible paths");
        };

        // Save all other paths.
        for (bb, constraint) in possible_paths {
            self.fork_and_branch(bb, Some(constraint))?;
        }

        self.state.constraints.assert(&constraint);
        Ok(InstructionResult::Branch(target))
    }

    fn indirect_br(&mut self, _i: &instruction::IndirectBr) -> Result<InstructionResult> {
        todo!("indirect_br")
    }

    fn invoke(&mut self, _i: &instruction::Invoke) -> Result<InstructionResult> {
        todo!()
    }

    fn resume(&mut self, _i: &instruction::Resume) -> Result<InstructionResult> {
        todo!()
    }

    fn unreachable(&mut self, _i: &instruction::Unreachable) -> Result<InstructionResult> {
        debug!("{_i}");
        Ok(InstructionResult::AnalysisError(AnalysisError::Unreachable))
    }

    fn cleanup_ret(&mut self, _i: &instruction::CleanupRet) -> Result<InstructionResult> {
        todo!()
    }

    fn catch_ret(&mut self, _i: &instruction::CatchRet) -> Result<InstructionResult> {
        todo!()
    }

    fn catch_switch(&mut self, _i: &instruction::CatchSwitch) -> Result<InstructionResult> {
        todo!()
    }

    fn call_br(&mut self, _i: &instruction::CallBr) -> Result<InstructionResult> {
        todo!()
    }
}

/// Perform a binary operation on two operands, returning the result.
///
/// The input types must be either integers or a vector of integers. Vector operations are performed
/// on a per element basis.
///
/// TODO: No operations currently care about overflows and such.
pub(crate) fn binop<F>(
    state: &mut LLVMState,
    op0: &Value,
    op1: &Value,
    operation: F,
) -> Result<DExpr>
where
    F: Fn(&DExpr, &DExpr) -> DExpr,
{
    let lhs = state.get_expr(op0)?;
    let rhs = state.get_expr(op1)?;

    match (op0.ty(), op1.ty()) {
        // For simple integer types the result is trivial to do, just perform the operation.
        (Type::Integer(_), Type::Integer(_)) => Ok(operation(&lhs, &rhs)),

        // Some may support pointer operands, such as icmp, which work the same as integer ones.
        (Type::Pointer(_), Type::Pointer(_)) => Ok(operation(&lhs, &rhs)),

        // The docs do not really specify how the vector operations should work. But I'll assume it
        // is the operation on a per element basis.
        (Type::Vector(t0), Type::Vector(t1)) if !(t0.is_scalable() || t1.is_scalable()) => {
            let element_type = t0.element_type();
            let num_elements = t0.num_elements();
            let bits = bit_size(&element_type, state.project.ptr_size).unwrap();

            let rhs_element_type = t1.element_type();
            let rhs_num_elements = t1.num_elements();
            let rhs_bits = bit_size(&rhs_element_type, state.project.ptr_size).unwrap();

            assert_eq!(element_type, rhs_element_type);
            assert_eq!(num_elements, rhs_num_elements);
            assert_eq!(bits, rhs_bits);

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

        // These types should not appear in a binary operation.
        _ => Err(LLVMExecutorError::MalformedInstruction),
    }
}

pub(crate) fn bit_size(ty: &Type, ptr_size: u32) -> Result<u32> {
    match ty {
        Type::Void => Ok(0),
        Type::Integer(t) => Ok(t.bits()),
        Type::Float(t) => Ok(t.bits()),
        Type::Pointer(_) => Ok(ptr_size),
        Type::Vector(t) => {
            bit_size(&t.element_type(), ptr_size).map(|size| size * t.num_elements())
        }
        Type::Array(t) => {
            bit_size(&t.element_type(), ptr_size).map(|size| size * t.num_elements() as u32)
        }
        Type::Structure(t) => t.fields().into_iter().map(|f| bit_size(&f, ptr_size)).sum(),
        Type::OpaqueStructure => todo!("Cannot get size of opaque structure"),

        // TODO: How are these sized?
        Type::Function(_)
        | Type::Label
        | Type::Token
        | Type::Metadata
        | Type::X86Amx
        | Type::X86Mmx
        | Type::TargetExtension(_) => todo!("Getting size for {ty:?} is unsupported"),
    }
}

/// Calculates the size of the type in bytes.
pub(crate) fn byte_size(ty: &Type, ptr_size: u32) -> Result<u32> {
    let bit_size = bit_size(ty, ptr_size)?;
    let byte_size = to_bytes_u32(bit_size)?;
    Ok(byte_size)
}

/// Calculate `[start..end]` offset into an aggregate.
pub(crate) fn get_element_offset(
    state: &LLVMState,
    aggregate_type: Type,
    indices: &[u32],
) -> Result<(u32, u32)> {
    let mut aggregate_type = aggregate_type;

    let mut lower_bound = 0;
    for index in indices.iter().copied() {
        let (offset, inner_ty) = get_bit_offset_concrete(&aggregate_type, index, &state.project)?;

        lower_bound += offset;
        aggregate_type = inner_ty;
    }

    let ptr_size = state.project.ptr_size;
    let element_size = bit_size(&aggregate_type, ptr_size)?;
    let upper_bound = lower_bound + element_size;

    Ok((lower_bound, upper_bound))
}

/// Calculate the offset in bits from a concrete index.
pub(crate) fn get_bit_offset_concrete(
    ty: &Type,
    index: u32,
    project: &Project,
) -> Result<(u32, Type)> {
    let ptr_size = project.ptr_size;

    match ty {
        // Can this work with opaque pointers? Don't think so.
        Type::Pointer(_) => todo!(),

        Type::Vector(t) => {
            let element_type = t.element_type();
            let element_size = bit_size(&element_type, ptr_size).unwrap();
            Ok((element_size * index, element_type))
        }

        Type::Array(t) => {
            let element_type = t.element_type();
            let element_size = bit_size(&element_type, ptr_size).unwrap();
            Ok((element_size * index, element_type))
        }

        Type::Structure(t) => {
            let fields = t.fields();
            let offset = fields
                .iter()
                .take(index as usize)
                .map(|f| bit_size(f, ptr_size))
                .sum::<Result<u32>>()?;

            let inner_ty = fields
                .get(index as usize)
                .cloned()
                .ok_or(LLVMExecutorError::MalformedInstruction)?;

            Ok((offset, inner_ty))
        }
        Type::OpaqueStructure => Err(LLVMExecutorError::NoSize),

        // We cannot index into these types.
        Type::X86Amx
        | Type::X86Mmx
        | Type::Void
        | Type::Function(_)
        | Type::Integer(_)
        | Type::Float(_)
        | Type::Label
        | Type::Token
        | Type::Metadata
        | Type::TargetExtension(_) => panic!("Cannot index into type: {ty:?}"),
    }
}

/// Get the byte offset.
///
/// This checks that each offset is byte divisible. For struct offsets the expression must be
/// constant.
pub(crate) fn byte_offset(
    ty: &Type,
    index: &DExpr,
    ptr_size: u32,
    ctx: &DContext,
) -> Result<(DExpr, Type)> {
    match ty {
        Type::Pointer(_) => todo!(),

        Type::Vector(t) => {
            let element_type = t.element_type();
            let element_size = byte_size(&element_type, ptr_size)?;
            let element_size = ctx.from_u64(element_size.into(), index.len());
            Ok((element_size.mul(index), element_type))
        }

        Type::Array(t) => {
            let element_type = t.element_type();
            let element_size = byte_size(&element_type, ptr_size)?;
            let element_size = ctx.from_u64(element_size.into(), index.len());
            Ok((element_size.mul(index), element_type))
        }

        // For structs we have to collect the size of all the members until the index. Not supported
        // for non-constant indexes.
        //
        // With a symbol as an index we cannot index into structs, not without having to try
        // solutions and fork the state. So these are not supported.
        Type::Structure(t) => {
            let index_len = index.len();
            let Some(index) = index.get_constant() else {
                panic!("Cannot index into struct with non-constant index");
            };

            let fields = t.fields();
            let offset = fields
                .iter()
                .take(index as usize)
                .map(|f| byte_size(f, ptr_size))
                .sum::<Result<u32>>()?;

            let inner_ty = fields
                .get(index as usize)
                .cloned()
                .ok_or(LLVMExecutorError::MalformedInstruction)?;

            let offset = ctx.from_u64(offset.into(), index_len);
            Ok((offset, inner_ty))
        }
        Type::OpaqueStructure => Err(LLVMExecutorError::NoSize),

        // We cannot index into these types.
        Type::Void => todo!(),
        Type::Integer(_) => todo!(),
        Type::Float(_) => todo!(),
        Type::Function(_) => todo!(),
        Type::Label => todo!(),
        Type::Token => todo!(),
        Type::Metadata => todo!(),
        Type::X86Amx => todo!(),
        Type::X86Mmx => todo!(),
        Type::TargetExtension(_) => todo!(),
    }
}

/// Converts integers, pointers, or vectors.
///
/// Performs a conversion on either (int,int), (ptr,int), (int,ptr), or (vector,vector) with the
/// passed mapping function.
///
/// No type checking is done, if this is of interest they have to be checked before calling this
/// function.
pub(crate) fn convert_to_map<F>(
    state: &mut LLVMState,
    value: Value,
    to_type: &Type,
    map: F,
) -> Result<DExpr>
where
    F: Fn(DExpr, u32) -> DExpr,
{
    let value_type = value.ty();
    let value = state.get_expr(&value)?;

    match (value_type, to_type) {
        // Integer to integer conversion are done by trunc, zext, and sext. While the ptr->int,
        // int->ptr are done by ptrtoint and inttoptr. All these should be supported.
        (Type::Integer(_), Type::Integer(_))
        | (Type::Integer(_), Type::Pointer(_))
        | (Type::Pointer(_), Type::Integer(_)) => {
            let target_bits = bit_size(to_type, state.project.ptr_size)?;
            Ok(map(value, target_bits))
        }

        // Vectors are a bit more annoying, in that the elements have to be processed one by one.
        (Type::Vector(source), Type::Vector(target))
            if !(source.is_scalable() || target.is_scalable()) =>
        {
            let source_bits = bit_size(&source.element_type(), state.project.ptr_size)?;
            let target_bits = bit_size(&target.element_type(), state.project.ptr_size)?;

            let num_elements = source.num_elements();
            assert!(source_bits * num_elements == value.len());

            // Process each element one by one and concatenate the result.
            (0..num_elements)
                .map(|i| {
                    let low = i * source_bits;
                    let high = (i + 1) * source_bits - 1;
                    let symbol = value.slice(low, high);
                    map(symbol, target_bits)
                })
                .reduce(|acc, v| v.concat(&acc))
                .ok_or(LLVMExecutorError::MalformedInstruction)
        }

        // The other types should not appear for this instruction.
        _ => Err(LLVMExecutorError::MalformedInstruction),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(fn_name: &str) -> Vec<Option<i64>> {
        // let subscriber = tracing_subscriber::FmtSubscriber::builder()
        //     .with_max_level(tracing::Level::TRACE)
        //     .finish();
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("setting default subscriber failed");

        let path = format!("tests/unit_tests/instructions.bc");
        let project = Box::new(Project::from_path(&path).expect("Failed to created project"));
        let project = Box::leak(project);

        let context = Box::new(DContext::new());
        let context = Box::leak(context);
        let mut vm = VM::new(project, context, fn_name).expect("Failed to create VM");

        let mut path_results = Vec::new();
        while let Some((path_result, state)) = vm.run().expect("Failed to run path") {
            let result = match path_result {
                PathResult::Success(Some(value)) => {
                    let value = state
                        .constraints
                        .get_value(&value)
                        .expect("Failed to get concrete value");
                    let binary_str = value.to_binary_string();
                    Some(u128::from_str_radix(&binary_str, 2).unwrap() as i64)
                }
                PathResult::Success(None) => None,
                PathResult::Failure(error) => {
                    panic!("Did not expect any paths to fail, reason: {error:?}")
                }
                PathResult::Suppress => panic!("Did not expect any paths to be suppressed"),
            };
            path_results.push(result);
        }

        println!("{path_results:x?}");
        path_results
    }

    #[test]
    fn test_add() {
        let res = run("test_add");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(15));
    }

    // #[test]
    // fn test_add_overflow() {
    //     let res = run("test_add_overflow");
    //     assert_eq!(res.len(), 1);
    //     assert!(res[0].is_ok());
    //     assert_eq!(res[0], Some(15));
    // }

    #[test]
    fn test_add_vector() {
        let res = run("test_add_vector");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0000_006E_0000_0019));
    }

    #[test]
    fn test_sub() {
        let res = run("test_sub");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(10));
    }

    #[test]
    fn test_mul() {
        let res = run("test_mul");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(35));
    }

    #[test]
    fn test_udiv() {
        let res = run("test_udiv");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(20));
    }

    #[test]
    fn test_sdiv() {
        let res = run("test_sdiv");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(-20));
    }

    #[test]
    fn test_urem() {
        let res = run("test_urem");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(3));
    }

    #[test]
    fn test_srem() {
        let res = run("test_srem");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(3));
    }

    #[test]
    fn test_srem2() {
        let res = run("test_srem2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(-3));
    }

    #[test]
    fn test_and() {
        let res = run("test_and");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(48));
    }

    #[test]
    fn test_or() {
        let res = run("test_or");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(59));
    }

    #[test]
    fn test_xor() {
        let res = run("test_xor");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(11));
    }

    #[test]
    fn test_shl() {
        let res = run("test_shl");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(116));
    }

    #[test]
    fn test_lshr() {
        let res = run("test_lshr");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(29));
    }

    #[test]
    fn test_ashr() {
        let res = run("test_ashr");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(29));
    }

    #[test]
    fn test_ashr2() {
        let res = run("test_ashr2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(221));
    }

    #[test]
    fn test_extract_value_arr1() {
        let res = run("test_extract_value_arr1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(1));
    }

    #[test]
    fn test_extract_value_arr2() {
        let res = run("test_extract_value_arr2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(2));
    }

    #[test]
    fn test_extract_value_arr3() {
        let res = run("test_extract_value_arr3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(3));
    }

    #[test]
    fn test_extract_value_arr4() {
        let res = run("test_extract_value_arr4");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(4));
    }

    #[test]
    fn test_extract_value_struct1() {
        let res = run("test_extract_value_struct1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(4));
    }

    #[test]
    fn test_extract_value_struct2() {
        let res = run("test_extract_value_struct2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(10));
    }

    #[test]
    fn test_extract_value_struct3() {
        let res = run("test_extract_value_struct3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(1));
    }

    #[test]
    fn test_extract_value_struct4() {
        let res = run("test_extract_value_struct4");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(2));
    }

    #[test]
    fn test_insert_value_arr1() {
        let res = run("test_insert_value_arr1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0403020a));
    }

    #[test]
    fn test_insert_value_arr2() {
        let res = run("test_insert_value_arr2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x04030a01));
    }

    #[test]
    fn test_insert_value_arr3() {
        let res = run("test_insert_value_arr3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x040a0201));
    }

    #[test]
    fn test_insert_value_arr4() {
        let res = run("test_insert_value_arr4");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0a030201));
    }

    #[test]
    fn test_insert_value_arr5() {
        let res = run("test_insert_value_arr5");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0506070803040102));
    }

    #[test]
    fn test_insert_value_struct1() {
        let res = run("test_insert_value_struct1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x00020001000a000f));
    }

    #[test]
    fn test_insert_value_struct2() {
        let res = run("test_insert_value_struct2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x00020001000f0004));
    }

    #[test]
    fn test_insert_value_struct3() {
        let res = run("test_insert_value_struct3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0002000f000a0004));
    }

    #[test]
    fn test_insert_value_struct4() {
        let res = run("test_insert_value_struct4");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x000f0001000a0004));
    }

    #[test]
    fn test_load_store1() {
        let res = run("test_load_store1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x3456));
    }

    #[test]
    fn test_load_store2() {
        let res = run("test_load_store2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x56));
    }

    #[test]
    fn test_gep1() {
        let res = run("test_gep1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(1));
    }

    #[test]
    fn test_gep2() {
        let res = run("test_gep2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(2));
    }

    #[test]
    fn test_gep3() {
        let res = run("test_gep3");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(3));
    }

    #[test]
    fn test_gep4() {
        let res = run("test_gep4");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(4));
    }

    #[test]
    fn test_gep5() {
        let res = run("test_gep5");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(5));
    }

    #[test]
    fn test_gep6() {
        let res = run("test_gep6");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(6));
    }

    #[test]
    fn test_gep_arr() {
        let res = run("test_gep_arr");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(2));
    }

    #[test]
    fn test_gep_arr2() {
        let res = run("test_gep_arr2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(2));
    }

    #[test]
    fn test_bitcast1() {
        let res = run("test_bitcast1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x56781234));
    }

    #[test]
    fn test_bitcast2() {
        let res = run("test_bitcast2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x78563412));
    }

    #[test]
    fn test_trunc() {
        let res = run("test_trunc");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0xCD));
    }

    #[test]
    fn test_zext() {
        let res = run("test_zext");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x00FF));
    }

    #[test]
    fn test_zext_vec() {
        let res = run("test_zext_vec");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x000F00FF));
    }

    #[test]
    fn test_sext() {
        let res = run("test_sext");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0xFFFF));
    }

    #[test]
    fn test_inttoptr_trunc() {
        let res = run("test_inttoptr_trunc");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x5555666677778888));
    }

    #[test]
    fn test_inttoptr_noop() {
        let res = run("test_inttoptr_noop");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1111222233334444));
    }

    #[test]
    fn test_inttoptr_extend() {
        let res = run("test_inttoptr_extend");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0000000011112222));
    }

    #[test]
    fn test_ptrtoint_trunc() {
        let res = run("test_ptrtoint_trunc");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x44));
    }

    #[test]
    fn test_ptrtoint_noop() {
        let res = run("test_ptrtoint_noop");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1111222233334444));
    }

    #[test]
    fn test_ptrtoint_vec_trunc() {
        let res = run("test_ptrtoint_vec_trunc");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x7777888833334444));
    }

    #[test]
    fn test_addrspacecast() {
        let res = run("test_addrspacecast");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1111222233334444));
    }

    #[test]
    fn test_icmp_eq_false() {
        let res = run("test_icmp_eq_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0));
    }

    #[test]
    fn test_icmp_eq_true() {
        let res = run("test_icmp_eq_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1));
    }

    #[test]
    fn test_icmp_ne_false() {
        let res = run("test_icmp_ne_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0));
    }

    #[test]
    fn test_icmp_ne_true() {
        let res = run("test_icmp_ne_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1));
    }

    #[test]
    fn test_icmp_ugt_false() {
        let res = run("test_icmp_ugt_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0));
    }

    #[test]
    fn test_icmp_ugt_true() {
        let res = run("test_icmp_ugt_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1));
    }

    #[test]
    fn test_icmp_uge_false() {
        let res = run("test_icmp_uge_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0));
    }

    #[test]
    fn test_icmp_uge_true() {
        let res = run("test_icmp_uge_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1));
    }

    #[test]
    fn test_icmp_ult_false() {
        let res = run("test_icmp_ult_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0));
    }

    #[test]
    fn test_icmp_ult_true() {
        let res = run("test_icmp_ult_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1));
    }

    #[test]
    fn test_icmp_ule_false() {
        let res = run("test_icmp_ule_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0));
    }

    #[test]
    fn test_icmp_ule_true() {
        let res = run("test_icmp_ule_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1));
    }

    #[test]
    fn test_icmp_sgt_false() {
        let res = run("test_icmp_sgt_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0));
    }

    #[test]
    fn test_icmp_sgt_true() {
        let res = run("test_icmp_sgt_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1));
    }

    #[test]
    fn test_icmp_sge_false() {
        let res = run("test_icmp_sge_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0));
    }

    #[test]
    fn test_icmp_sge_true() {
        let res = run("test_icmp_sge_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1));
    }

    #[test]
    fn test_icmp_slt_false() {
        let res = run("test_icmp_slt_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0));
    }

    #[test]
    fn test_icmp_slt_true() {
        let res = run("test_icmp_slt_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1));
    }

    #[test]
    fn test_icmp_sle_false() {
        let res = run("test_icmp_sle_false");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x0));
    }

    #[test]
    fn test_icmp_sle_true() {
        let res = run("test_icmp_sle_true");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x1));
    }

    #[test]
    fn test_icmp_eq_vec() {
        let res = run("test_icmp_eq_vec");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x5));
    }

    #[test]
    fn test_phi1() {
        let res = run("test_phi1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0xab));
    }

    #[test]
    fn test_phi2() {
        let res = run("test_phi2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0xcd));
    }

    #[test]
    fn test_select1() {
        let res = run("test_select1");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0xaa));
    }

    #[test]
    fn test_select2() {
        let res = run("test_select2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0xbb));
    }

    #[test]
    fn test_select_vec_values() {
        let res = run("test_select_vec_values");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x000000dd000000cc));
    }

    #[test]
    fn test_select_vec_cond() {
        let res = run("test_select_vec_cond");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x000000dd000000aa));
    }

    #[test]
    fn test_call() {
        let res = run("test_call");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0xabcd));
    }

    #[test]
    fn test_vector_constant() {
        let res = run("test_vector_constant");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x5321));
    }

    #[test]
    fn test_vector_constant2() {
        let res = run("test_vector_constant2");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], Some(0x04030201));
    }
}

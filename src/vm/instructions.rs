use llvm_ir::instruction::{BinaryOp, HasResult, Instruction};
use llvm_ir::{instruction, terminator, Terminator, Type};
use log::{debug, warn};

use super::{ReturnValue, VMError, VM};
use crate::hooks::FnInfo;
use crate::project::FunctionType;
use crate::solver::BinaryOperation;
use crate::traits::*;
use crate::vm::Callsite;
use crate::vm::Location;
use crate::vm::Result;

impl<'a> VM<'a> {
    pub fn process_instruction(&mut self, instr: &'a Instruction) -> Result<()> {
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

    pub fn process_terminator(&mut self, terminator: &Terminator) -> Result<ReturnValue> {
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

    // -------------------------------------------------------------------------
    // Terminator Instructions
    // -------------------------------------------------------------------------

    fn ret(&mut self, instr: &terminator::Ret) -> Result<ReturnValue> {
        debug!("{}", instr);
        let ret_val = if let Some(op) = &instr.return_operand {
            let value = self.state.get_var(op).unwrap();
            ReturnValue::Value(value)
        } else {
            ReturnValue::Void
        };

        // On `ret` the variable scope that was entered when calling/invoking must be destroyed.

        // Note that this works even in error cases, if we encounter an error this path is dead.
        // Execution is never resumed of a path that encountered an error.
        self.state.vars.leave_scope();
        Ok(ret_val)
    }

    fn br(&mut self, instr: &terminator::Br) -> Result<ReturnValue> {
        debug!("{}", instr);
        self.branch(&instr.dest)
    }

    fn condbr(&mut self, instr: &terminator::CondBr) -> Result<ReturnValue> {
        debug!("{}", instr);

        let cond = self.state.get_var(&instr.condition).unwrap();
        let true_possible = self.solver.is_sat_with_constraint(&cond).unwrap();
        let false_possible = self.solver.is_sat_with_constraint(&cond.not()).unwrap();

        let target = match (true_possible, false_possible) {
            (true, true) => {
                // Explore true first, then backtrack to false.
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

    fn switch(&mut self, instr: &terminator::Switch) -> Result<ReturnValue> {
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
            default_cond = default_cond.and(&value.ne(&path_cond)).into();

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
            self.solver.assert(&cond);
            self.branch(target)
        } else {
            // Should never happen, since if we have no paths at all, the
            // default condition should be just `true`.
            unreachable!();
        }
    }

    fn indirectbr(&mut self, instr: &terminator::IndirectBr) -> Result<ReturnValue> {
        debug!("{}", instr);
        todo!()
    }

    fn invoke(&mut self, instr: &terminator::Invoke) -> Result<ReturnValue> {
        debug!("{}", instr);
        todo!()
    }

    fn callbr(&mut self, instr: &terminator::CallBr) -> Result<ReturnValue> {
        debug!("{}", instr);
        todo!()
    }

    fn resume(&mut self, instr: &terminator::Resume) -> Result<ReturnValue> {
        debug!("{}", instr);
        todo!()
    }

    fn catchswitch(&mut self, instr: &terminator::CatchSwitch) -> Result<ReturnValue> {
        debug!("{}", instr);
        todo!()
    }

    fn catchret(&mut self, instr: &terminator::CatchRet) -> Result<ReturnValue> {
        debug!("{}", instr);
        todo!()
    }

    fn cleanupret(&mut self, instr: &terminator::CleanupRet) -> Result<ReturnValue> {
        debug!("{}", instr);
        todo!()
    }

    /// Unreachable returns an [`VMError::UnreachableInstruction`] error.
    ///
    /// The `unreachable` instruction as it name says, should be unreachable.
    /// If this is called, it is most likely an error in the interpreter.
    fn unreachable(&mut self, instr: &terminator::Unreachable) -> Result<ReturnValue> {
        debug!("{}", instr);
        Err(VMError::UnreachableInstruction)
    }

    // -------------------------------------------------------------------------
    // Unary Operations
    // -------------------------------------------------------------------------

    fn fneg(&mut self, instr: &instruction::FNeg) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    // -------------------------------------------------------------------------
    // Binary Operations
    // -------------------------------------------------------------------------

    // fn binop(
    //     &mut self,
    //     dst: &impl HasResult,
    //     lhs: &Operand,
    //     rhs: &Operand,
    //     op: BinaryOperation,
    // ) -> Result<()> {
    //     // TODO: Could check that types match?
    //     // let lhs_ty = self.state.type_of(lhs);
    //     // let rhs_ty = self.state.type_of(rhs);

    //     let lhs_bv = self.state.get_bv(lhs).unwrap();
    //     let rhs_bv = self.state.get_bv(rhs).unwrap();
    //     let result_bv = lhs_bv.binary_op(rhs_bv, op);

    //     self.assign(dst, result_bv)
    // }

    fn add(&mut self, instr: &instruction::Add) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &mut self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BinaryOperation::Add,
        )?;
        self.assign(instr, result)
    }

    fn fadd(&mut self, instr: &instruction::FAdd) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn sub(&mut self, instr: &instruction::Sub) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &mut self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BinaryOperation::Sub,
        )?;
        self.assign(instr, result)
    }

    fn fsub(&mut self, instr: &instruction::FSub) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn mul(&mut self, instr: &instruction::Mul) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &mut self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BinaryOperation::Mul,
        )?;
        self.assign(instr, result)
    }

    fn fmul(&mut self, instr: &instruction::FMul) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn udiv(&mut self, instr: &instruction::UDiv) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &mut self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BinaryOperation::UDiv,
        )?;
        self.assign(instr, result)
    }

    fn sdiv(&mut self, instr: &instruction::SDiv) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &mut self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BinaryOperation::SDiv,
        )?;
        self.assign(instr, result)
    }

    fn fdiv(&mut self, instr: &instruction::FDiv) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn urem(&mut self, instr: &instruction::URem) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &mut self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BinaryOperation::URem,
        )?;
        self.assign(instr, result)
    }

    fn srem(&mut self, instr: &instruction::SRem) -> Result<()> {
        debug!("{}", instr);
        let result = binop(
            &mut self.state,
            instr.get_operand0(),
            instr.get_operand1(),
            BinaryOperation::SRem,
        )?;
        self.assign(instr, result)
    }

    fn frem(&mut self, instr: &instruction::FRem) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    // -------------------------------------------------------------------------
    // Bitwise Binary Operations
    // -------------------------------------------------------------------------

    fn shl(&mut self, instr: &instruction::Shl) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn lshr(&mut self, instr: &instruction::LShr) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn ashr(&mut self, instr: &instruction::AShr) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn and(&mut self, instr: &instruction::And) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn or(&mut self, instr: &instruction::Or) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn xor(&mut self, instr: &instruction::Xor) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    // -------------------------------------------------------------------------
    // Vector Operations
    // -------------------------------------------------------------------------

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

    // -------------------------------------------------------------------------
    // Aggregate Operations
    // -------------------------------------------------------------------------

    fn extractvalue(&mut self, instr: &instruction::ExtractValue) -> Result<()> {
        debug!("{}", instr);
        let value = extract_value(&mut self.state, &instr.aggregate, &instr.indices)?;
        self.assign(instr, value)
    }

    fn insertvalue(&mut self, instr: &instruction::InsertValue) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    // -------------------------------------------------------------------------
    // Memory access and Addressing Operations
    // -------------------------------------------------------------------------

    /// Allocate memory on the stack frame of the currently executing function.
    /// The memory is automatically cleaned up when the function returns.
    ///
    /// Reference: https://llvm.org/docs/LangRef.html#alloca-instruction
    fn alloca(&mut self, instr: &instruction::Alloca) -> Result<()> {
        debug!("{}", instr);

        let num_elements = instr.num_elements.to_value()?;
        let element_size = instr.allocated_type.size(self.project).unwrap();

        // let num_elements = u64_from_operand(&instr.num_elements).unwrap() as usize;
        // let element_size = size_in_bits(&instr.allocated_type, self.project).unwrap();
        let mut allocation_size = element_size * num_elements;

        if allocation_size == 0 {
            // panic!("zero sized alloca is not supported yet");
            warn!("zero sized alloca");
            allocation_size = self.project.ptr_size;
        }

        let addr = self
            .state
            .stack_alloc(allocation_size, instr.alignment as u64)
            .unwrap();

        self.assign(instr, addr)
    }

    /// Load reads a value from memory.
    ///
    /// Reference: https://llvm.org/docs/LangRef.html#load-instruction
    fn load(&mut self, instr: &instruction::Load) -> Result<()> {
        debug!("{}", instr);

        let addr = self.state.get_var(&instr.address).unwrap();
        let dst_ty = self.state.type_of(instr);
        let dst_size = dst_ty.size(self.project).unwrap() as u32;
        // let dst_size = size_in_bits(&dst_ty, self.project).unwrap() as u32;
        // println!("dst size: {}", dst_size);

        let value = self.state.mem.read(&addr, dst_size).unwrap();
        // println!("got value: {:?}", value);
        self.state
            .assign_bv(instr.get_result().clone(), value)
            .unwrap();

        Ok(())
    }

    /// Store writes to a value to memory.
    ///
    /// Accepts a value which will be written to the passed pointer address.
    ///
    /// Reference: https://llvm.org/docs/LangRef.html#store-instruction
    fn store(&mut self, instr: &instruction::Store) -> Result<()> {
        debug!("{}", instr);

        let value = self.state.get_var(&instr.value).unwrap();
        let addr = self.state.get_var(&instr.address).unwrap();
        self.state.mem.write(&addr, value).unwrap();

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

    /// GetElementPtr calculates the offset into an array or struct from a base
    /// pointer.
    ///
    /// Reference: https://llvm.org/docs/LangRef.html#getelementptr-instruction
    fn getelementptr(&mut self, instr: &instruction::GetElementPtr) -> Result<()> {
        debug!("{}", instr);
        let target_address = gep(
            &mut self.state,
            &instr.address,
            instr.indices.iter().map(|op| op.into()),
            instr.in_bounds,
        )?;

        self.assign(instr, target_address)
    }

    // -------------------------------------------------------------------------
    // Conversion Operations
    // -------------------------------------------------------------------------

    fn trunc(&mut self, instr: &instruction::Trunc) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    /// Zero extends a value to the destination type width.
    ///
    /// Requires the destination type to be non-decreasing compared to the
    /// source type.
    ///
    /// Reference: https://llvm.org/docs/LangRef.html#zext-to-instruction
    fn zext(&mut self, instr: &instruction::ZExt) -> Result<()> {
        debug!("{}", instr);

        // Required that both type be integers, or vector with the same number
        // of integers.
        let ty0 = self.state.type_of(&instr.operand);
        match (ty0.as_ref(), instr.to_type.as_ref()) {
            (Type::IntegerType { bits: n }, Type::IntegerType { bits: m }) => {
                assert!(n < m);
                let value = self.state.get_var(&instr.operand).unwrap();
                assert_eq!(*n, value.len());

                let value = value.zero_ext(*m);
                self.state
                    .assign_bv(instr.get_result().clone(), value)
                    .unwrap();
            }
            (
                Type::VectorType {
                    num_elements: n, ..
                },
                Type::VectorType {
                    num_elements: m, ..
                },
            ) if n == m => {
                // TODO: Check that ty0 size < ty1 size.
                todo!()
            }
            _ => panic!("invalid instruction"),
        }
        Ok(())
    }

    fn sext(&mut self, instr: &instruction::SExt) -> Result<()> {
        debug!("{}", instr);

        let ty0 = self.state.type_of(&instr.operand);
        match (ty0.as_ref(), instr.to_type.as_ref()) {
            (Type::IntegerType { bits: n }, Type::IntegerType { bits: m }) => {
                assert!(n < m);
                let value = self.state.get_var(&instr.operand).unwrap();
                assert_eq!(*n, value.len());

                let value = value.sign_ext(*m);
                self.state
                    .assign_bv(instr.get_result().clone(), value)
                    .unwrap();
            }
            (
                Type::VectorType {
                    num_elements: n, ..
                },
                Type::VectorType {
                    num_elements: m, ..
                },
            ) if n == m => {
                // TODO: Check that ty0 size < ty1 size.
                todo!()
            }
            _ => panic!("invalid instruction"),
        }
        Ok(())
    }

    fn fptrunc(&mut self, instr: &instruction::FPTrunc) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn fpext(&mut self, instr: &instruction::FPExt) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn fptoui(&mut self, instr: &instruction::FPToUI) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn fptosi(&mut self, instr: &instruction::FPToSI) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn uitofp(&mut self, instr: &instruction::UIToFP) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn sitofp(&mut self, instr: &instruction::SIToFP) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn ptrtoint(&mut self, instr: &instruction::PtrToInt) -> Result<()> {
        debug!("{}", instr);
        let bv = cast_to(&mut self.state, &instr.to_type, &instr.operand)?;
        self.assign(instr, bv)
    }

    fn inttoptr(&mut self, instr: &instruction::IntToPtr) -> Result<()> {
        debug!("{}", instr);
        let bv = cast_to(&mut self.state, &instr.to_type, &instr.operand)?;
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
        let bv = cast_to(&mut self.state, &instr.to_type, &instr.operand)?;
        self.assign(instr, bv)
    }

    fn addrspacecast(&mut self, instr: &instruction::AddrSpaceCast) -> Result<()> {
        debug!("{}", instr);
        let bv = cast_to(&mut self.state, &instr.to_type, &instr.operand)?;
        self.assign(instr, bv)
    }

    // -------------------------------------------------------------------------
    // Other Operations
    // -------------------------------------------------------------------------

    fn icmp(&mut self, instr: &instruction::ICmp) -> Result<()> {
        debug!("{}", instr);
        let result = icmp(
            &mut self.state,
            &instr.operand0,
            &instr.operand1,
            instr.predicate,
        )?;
        self.assign(instr, result)
    }

    fn fcmp(&mut self, instr: &instruction::FCmp) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn phi(&mut self, instr: &instruction::Phi) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn select(&mut self, instr: &instruction::Select) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn freeze(&mut self, instr: &instruction::Freeze) -> Result<()> {
        debug!("{}", instr);
        todo!()
    }

    fn call(&mut self, instr: &'a instruction::Call) -> Result<()> {
        debug!("{}", instr);

        let fn_name = self.resolve_function(&instr.function)?;
        debug!("resolved function: {}", fn_name);

        let ret_val = match self
            .project
            .get_function(&fn_name, self.state.current_loc.module)?
        {
            FunctionType::Hook(hook) => {
                let info = FnInfo {
                    arguments: instr.arguments.clone(),
                    return_attrs: instr.return_attributes.clone(),
                    fn_attrs: instr.function_attributes.clone(),
                };
                hook(self, info)?
            }
            FunctionType::Function { function, module } => {
                let arguments = instr
                    .arguments
                    .iter()
                    .map(|(op, _)| self.state.get_var(op))
                    .collect::<Result<Vec<_>>>()?;

                // Create new location at the start of function to call, and store our current
                // position in the callstack so we can return here later.
                let mut new_location = Location::new(module, function);
                std::mem::swap(&mut new_location, &mut self.state.current_loc);

                let callsite = Callsite::from_call(new_location, instr);
                self.state.callstack.push(callsite);

                let ret_val = self.call_fn(function, arguments)?;

                // Restore callsite.
                let callsite = self.state.callstack.pop().unwrap();
                self.state.current_loc = callsite.location;

                ret_val
            }
        };

        // Assign return value
        let ret_val = match ret_val {
            ReturnValue::Value(v) => Some(v),
            ReturnValue::Void => None,
            // ReturnValue::Throw(_) => todo!("support throw in calls"),
            // ReturnValue::Abort => return Err(VMError::Abort),
        };

        if let Some(name) = instr.dest.clone() {
            match ret_val {
                Some(bv) => self.state.assign_bv(name, bv).unwrap(),
                // None => return Err(anyhow!("call expected return value, got void")),
                None => panic!("call expected return value, but got void"),
            };
        }

        Ok(())
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

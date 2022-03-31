use llvm_ir::instruction::{BinaryOp, HasResult, Instruction, UnaryOp};
use llvm_ir::{instruction, terminator, IntPredicate, Operand, Terminator, Type};
use log::{trace, warn};

use super::{ReturnValue, VMError, VM};
use crate::hooks::FnInfo;
use crate::llvm::{
    const_to_bv, get_inner_type, get_offset_in_bits, size_in_bits, size_in_bytes, u64_from_operand,
};
use crate::project::FunctionType;
use crate::solver::BinaryOperation;
use crate::vm::location::Location;
use crate::vm::Callsite;
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
        trace!("{}", instr);
        let ret_val = if let Some(op) = &instr.return_operand {
            let value = self.state.get_bv_from_operand(op).unwrap();
            ReturnValue::Return(value)
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
        trace!("{}", instr);
        self.branch(&instr.dest)
    }

    fn condbr(&mut self, instr: &terminator::CondBr) -> Result<ReturnValue> {
        trace!("{}", instr);

        let cond = self.state.get_bv_from_operand(&instr.condition).unwrap();
        let true_possible = self.solver.is_sat_with_constraint(&cond).unwrap();
        let false_possible = self
            .solver
            .is_sat_with_constraint(&cond.not().into())
            .unwrap();

        let target = match (true_possible, false_possible) {
            (true, true) => {
                trace!("condbr: true and false possible");
                // Explore true first, then backtrack to false.
                self.save_backtracking_path(&instr.false_dest, Some(cond.not().into()))?;
                cond.assert();
                &instr.true_dest
            }
            (true, false) => {
                trace!("condbr: true possible");
                //cond.assert();
                &instr.true_dest
            }
            (false, true) => {
                trace!("condbr: false possible");
                //cond.not().assert();
                &instr.false_dest
            }
            (false, false) => {
                trace!("condbr: neither possible");
                return Err(VMError::Unsat);
            }
        };

        self.branch(target)
    }

    fn switch(&mut self, instr: &terminator::Switch) -> Result<ReturnValue> {
        trace!("{}", instr);
        let value = self.state.get_bv_from_operand(&instr.operand).unwrap();

        // A switch can pick any paths, the default path can be chosen if value
        // can be something other than any of the switches.
        let mut default_cond = self.solver.bv_from_bool(true);

        let mut paths = Vec::new();

        // Check if any of the non-default cases can be reached.
        for (constant, target) in instr.dests.iter() {
            let path_cond = const_to_bv(constant, &mut self.state).unwrap();

            // Build default condition.
            default_cond = default_cond.and(&value._ne(&path_cond)).into();

            let cond = value._eq(&path_cond);
            if self.solver.is_sat_with_constraint(&cond).unwrap() {
                trace!("switch: path {} possible", target);
                paths.push((target, cond));
            }
        }

        // Check if the default case can be reached.
        if self.solver.is_sat_with_constraint(&default_cond).unwrap() {
            trace!("switch: default path possible");
            paths.push((&instr.default_dest, default_cond));
        }

        for (target, cond) in paths.iter().skip(1).cloned() {
            self.save_backtracking_path(target, Some(cond))?;
        }

        if let Some((target, cond)) = paths.first() {
            cond.assert();
            self.branch(target)
        } else {
            // Should never happen, since if we have no paths at all, the
            // default condition should be just `true`.
            unreachable!();
        }
    }

    fn indirectbr(&mut self, instr: &terminator::IndirectBr) -> Result<ReturnValue> {
        trace!("{}", instr);
        todo!()
    }

    fn invoke(&mut self, instr: &terminator::Invoke) -> Result<ReturnValue> {
        trace!("{}", instr);
        todo!()
    }

    fn callbr(&mut self, instr: &terminator::CallBr) -> Result<ReturnValue> {
        trace!("{}", instr);
        todo!()
    }

    fn resume(&mut self, instr: &terminator::Resume) -> Result<ReturnValue> {
        trace!("{}", instr);
        todo!()
    }

    fn catchswitch(&mut self, instr: &terminator::CatchSwitch) -> Result<ReturnValue> {
        trace!("{}", instr);
        todo!()
    }

    fn catchret(&mut self, instr: &terminator::CatchRet) -> Result<ReturnValue> {
        trace!("{}", instr);
        todo!()
    }

    fn cleanupret(&mut self, instr: &terminator::CleanupRet) -> Result<ReturnValue> {
        trace!("{}", instr);
        todo!()
    }

    /// Unreachable returns an [`VMError::UnreachableInstruction`] error.
    ///
    /// The `unreachable` instruction as it name says, should be unreachable.
    /// If this is called, it is most likely an error in the interpreter.
    fn unreachable(&mut self, instr: &terminator::Unreachable) -> Result<ReturnValue> {
        trace!("{}", instr);
        Err(VMError::UnreachableInstruction)
    }

    // -------------------------------------------------------------------------
    // Unary Operations
    // -------------------------------------------------------------------------

    fn fneg(&mut self, instr: &instruction::FNeg) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    // -------------------------------------------------------------------------
    // Binary Operations
    // -------------------------------------------------------------------------

    fn binop(
        &mut self,
        dst: &impl HasResult,
        lhs: &Operand,
        rhs: &Operand,
        op: BinaryOperation,
    ) -> Result<()> {
        //let lhs_ty = self.state.type_of(lhs);
        //let rhs_ty = self.state.type_of(rhs);

        let lhs_bv = self.state.get_bv_from_operand(lhs).unwrap();
        let rhs_bv = self.state.get_bv_from_operand(rhs).unwrap();
        let result_bv = lhs_bv.binary_op(rhs_bv, op);

        let name = dst.get_result().clone();
        self.state.assign_bv(name, result_bv).unwrap();

        Ok(())
    }

    fn add(&mut self, instr: &instruction::Add) -> Result<()> {
        trace!("{}", instr);
        self.binop(
            instr,
            instr.get_operand0(),
            instr.get_operand1(),
            BinaryOperation::Add,
        )
    }

    fn fadd(&mut self, instr: &instruction::FAdd) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn sub(&mut self, instr: &instruction::Sub) -> Result<()> {
        trace!("{}", instr);
        self.binop(
            instr,
            instr.get_operand0(),
            instr.get_operand1(),
            BinaryOperation::Sub,
        )
    }

    fn fsub(&mut self, instr: &instruction::FSub) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn mul(&mut self, instr: &instruction::Mul) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn fmul(&mut self, instr: &instruction::FMul) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn udiv(&mut self, instr: &instruction::UDiv) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn sdiv(&mut self, instr: &instruction::SDiv) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn fdiv(&mut self, instr: &instruction::FDiv) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn urem(&mut self, instr: &instruction::URem) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn srem(&mut self, instr: &instruction::SRem) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn frem(&mut self, instr: &instruction::FRem) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    // -------------------------------------------------------------------------
    // Bitwise Binary Operations
    // -------------------------------------------------------------------------

    fn shl(&mut self, instr: &instruction::Shl) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn lshr(&mut self, instr: &instruction::LShr) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn ashr(&mut self, instr: &instruction::AShr) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn and(&mut self, instr: &instruction::And) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn or(&mut self, instr: &instruction::Or) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn xor(&mut self, instr: &instruction::Xor) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    // -------------------------------------------------------------------------
    // Vector Operations
    // -------------------------------------------------------------------------

    fn extractelement(&mut self, instr: &instruction::ExtractElement) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn insertelement(&mut self, instr: &instruction::InsertElement) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn shufflevector(&mut self, instr: &instruction::ShuffleVector) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    // -------------------------------------------------------------------------
    // Aggregate Operations
    // -------------------------------------------------------------------------

    fn extractvalue(&mut self, instr: &instruction::ExtractValue) -> Result<()> {
        trace!("{}", instr);

        let value = self.state.get_bv_from_operand(&instr.aggregate).unwrap();

        let mut base_ty = self.state.type_of(&instr.aggregate);
        let mut total_offset = 0;
        for index in instr.indices.iter() {
            let (offset, ty) = get_offset_in_bits(&base_ty, *index as usize, self.project);
            base_ty = ty;
            total_offset += offset;
        }

        let offset_high = total_offset + size_in_bits(&base_ty, self.project).unwrap();
        assert!(value.get_width() >= offset_high as u32);

        let value = value.slice(offset_high as u32 - 1, total_offset as u32);

        self.state
            .assign_bv(instr.get_result().clone(), value.into())
            .unwrap();
        Ok(())
    }

    fn insertvalue(&mut self, instr: &instruction::InsertValue) -> Result<()> {
        trace!("{}", instr);
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
        trace!("{}", instr);

        let num_elements = u64_from_operand(&instr.num_elements).unwrap() as usize;
        let element_size = size_in_bits(&instr.allocated_type, self.project).unwrap();
        let mut allocation_size = element_size * num_elements;

        if allocation_size == 0 {
            // panic!("zero sized alloca is not supported yet");
            warn!("zero sized alloca");
            allocation_size = self.project.ptr_size;
        }

        let addr = self
            .state
            .stack_alloc(allocation_size, instr.alignment as usize)
            .unwrap();

        self.assign(instr, addr)
    }

    /// Load reads a value from memory.
    ///
    /// Reference: https://llvm.org/docs/LangRef.html#load-instruction
    fn load(&mut self, instr: &instruction::Load) -> Result<()> {
        trace!("{}", instr);

        let addr = self.state.get_bv_from_operand(&instr.address).unwrap();
        let dst_ty = self.state.type_of(instr);
        let dst_size = size_in_bits(&dst_ty, self.project).unwrap() as u32;
        println!("dst size: {}", dst_size);

        let value = self.state.mem.read(&addr, dst_size).unwrap();
        println!("got value: {:?}", value);
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
        trace!("{}", instr);

        let value = self.state.get_bv_from_operand(&instr.value).unwrap();
        let addr = self.state.get_bv_from_operand(&instr.address).unwrap();
        self.state.mem.write(&addr, value).unwrap();

        Ok(())
    }

    fn fence(&mut self, instr: &instruction::Fence) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn cmpxchg(&mut self, instr: &instruction::CmpXchg) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn atomicrmw(&mut self, instr: &instruction::AtomicRMW) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    /// GetElementPtr calculates the offset into an array or struct from a base
    /// pointer.
    ///
    /// Reference: https://llvm.org/docs/LangRef.html#getelementptr-instruction
    fn getelementptr(&mut self, instr: &instruction::GetElementPtr) -> Result<()> {
        trace!("{}", instr);
        // Currently, symbolic values are supported except in the case of struct
        // field addressing. This require concretization which could be support
        // for at least a few values.
        //
        // Hence, we check if the current type is a struct. And if it is, the
        // operand is required to be a constant, for now.
        let base_addr = self.state.get_bv_from_operand(&instr.address).unwrap();
        let ptr_size = base_addr.get_width();

        let value_ty = self.state.type_of(&instr.address);
        let value_size = size_in_bytes(&value_ty, self.project).unwrap().unwrap() as u64;
        let value_size = self.solver.bv_from_u64(value_size, ptr_size);
        let upper_bound = base_addr.add(&value_size);

        let target_addr = base_addr.clone();

        // The offsets modifies the address ptr, and this is the type of what
        // is currently pointed to.
        let mut curr_ty = self.state.type_of(&instr.address);
        for index in instr.indices.iter() {
            let is_struct = matches!(
                curr_ty.as_ref(),
                Type::NamedStructType { .. } | Type::StructType { .. }
            );

            let (offset, ty) = if is_struct {
                // Concrete indexing into a struct.
                let index = u64_from_operand(index).unwrap() as usize;
                let (offset, ty) = get_offset_in_bits(&curr_ty, index, self.project);

                let offset_bytes = (offset / 8) as u64;
                let offset = self.solver.bv_from_u64(offset_bytes, ptr_size);

                (offset, ty)
            } else {
                // Symbolic index. We cannot support struct indexing here, since
                // we calculate the offset as size of type * index, which won't
                // offset correctly for structs.
                let index = self.state.get_bv_from_operand(index).unwrap();
                let index = index.zero_ext(ptr_size);

                let bytes = size_in_bytes(&curr_ty, self.project).unwrap().unwrap() as u64;
                let bytes = self.solver.bv_from_u64(bytes, ptr_size);

                let ty = get_inner_type(&curr_ty, self.project).unwrap();

                let offset = bytes.mul(&index).into();
                (offset, ty)
            };

            target_addr.add(&offset);
            curr_ty = ty;
        }

        // Check that the target address is in bounds.
        let is_below = target_addr.ult(&base_addr);
        let is_above = target_addr.ugte(&upper_bound);
        let out_of_bounds = is_below.or(&is_above).into();
        if self.solver.is_sat_with_constraint(&out_of_bounds).unwrap() {
            out_of_bounds.assert();
            let sol = self.solver.get_solutions_for_bv(&base_addr, 1).unwrap();
            match sol {
                crate::Solutions::None => println!("==== no solution"),
                crate::Solutions::Exactly(n) => println!("==== exact {:?}", n),
                crate::Solutions::AtLeast(n) => println!("==== atleast {:?}", n),
            }
            // panic!("not in bounds");
            return Err(VMError::OutOfBounds);
        }

        self.assign(instr, target_addr)
    }

    // -------------------------------------------------------------------------
    // Conversion Operations
    // -------------------------------------------------------------------------

    fn trunc(&mut self, instr: &instruction::Trunc) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    /// Zero extends a value to the destination type width.
    ///
    /// Requires the destination type to be non-decreasing compared to the
    /// source type.
    ///
    /// Reference: https://llvm.org/docs/LangRef.html#zext-to-instruction
    fn zext(&mut self, instr: &instruction::ZExt) -> Result<()> {
        trace!("{}", instr);

        // Required that both type be integers, or vector with the same number
        // of integers.
        let ty0 = self.state.type_of(&instr.operand);
        match (ty0.as_ref(), instr.to_type.as_ref()) {
            (Type::IntegerType { bits: n }, Type::IntegerType { bits: m }) => {
                assert!(n < m);
                let value = self.state.get_bv_from_operand(&instr.operand).unwrap();
                assert_eq!(*n, value.get_width());

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
        trace!("{}", instr);

        let ty0 = self.state.type_of(&instr.operand);
        match (ty0.as_ref(), instr.to_type.as_ref()) {
            (Type::IntegerType { bits: n }, Type::IntegerType { bits: m }) => {
                assert!(n < m);
                let value = self.state.get_bv_from_operand(&instr.operand).unwrap();
                assert_eq!(*n, value.get_width());

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
        trace!("{}", instr);
        todo!()
    }

    fn fpext(&mut self, instr: &instruction::FPExt) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn fptoui(&mut self, instr: &instruction::FPToUI) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn fptosi(&mut self, instr: &instruction::FPToSI) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn uitofp(&mut self, instr: &instruction::UIToFP) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn sitofp(&mut self, instr: &instruction::SIToFP) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn ptrtoint(&mut self, instr: &instruction::PtrToInt) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn inttoptr(&mut self, instr: &instruction::IntToPtr) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    /// Bitcast casts a value to to resulting type without chaning any of the
    /// bits.
    ///
    /// Essentially a no-op, the implementation only stores the source value
    /// with the destination name and type.
    ///
    /// Reference: https://llvm.org/docs/LangRef.html#bitcast-to-instruction
    fn bitcast(&mut self, instr: &instruction::BitCast) -> Result<()> {
        trace!("{}", instr);
        let src_bv = self.state.get_bv_from_operand(instr.get_operand()).unwrap();
        self.assign(instr, src_bv)
    }

    fn addrspacecast(&mut self, instr: &instruction::AddrSpaceCast) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    // -------------------------------------------------------------------------
    // Other Operations
    // -------------------------------------------------------------------------

    fn icmp(&mut self, instr: &instruction::ICmp) -> Result<()> {
        trace!("{}", instr);

        let lhs = self.state.get_bv_from_operand(&instr.operand0).unwrap();
        let rhs = self.state.get_bv_from_operand(&instr.operand1).unwrap();
        let result = match instr.predicate {
            IntPredicate::EQ => lhs._eq(&rhs),
            IntPredicate::NE => lhs._ne(&rhs),
            IntPredicate::UGT => lhs.ugt(&rhs).into(),
            IntPredicate::UGE => lhs.ugte(&rhs).into(),
            IntPredicate::ULT => lhs.ult(&rhs).into(),
            IntPredicate::ULE => lhs.ulte(&rhs).into(),
            IntPredicate::SGT => lhs.sgt(&rhs).into(),
            IntPredicate::SGE => lhs.sgte(&rhs).into(),
            IntPredicate::SLT => lhs.slt(&rhs).into(),
            IntPredicate::SLE => lhs.slte(&rhs).into(),
        };

        self.assign(instr, result)
    }

    fn fcmp(&mut self, instr: &instruction::FCmp) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn phi(&mut self, instr: &instruction::Phi) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn select(&mut self, instr: &instruction::Select) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn freeze(&mut self, instr: &instruction::Freeze) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn call(&mut self, instr: &'a instruction::Call) -> Result<()> {
        trace!("{:?}", instr);

        let fn_name = self.resolve_function(&instr.function)?;
        println!("fn_name: {:?}", fn_name);

        let ret_val = match self.project.fn_by_name(&fn_name)? {
            FunctionType::Hook(hook) => {
                let info = FnInfo {
                    arguments: instr.arguments.clone(),
                    return_attrs: instr.return_attributes.clone(),
                    fn_attrs: instr.function_attributes.clone(),
                };
                hook(self, info).unwrap()
            }
            FunctionType::Function { function, module } => {
                let arguments = instr
                    .arguments
                    .iter()
                    .map(|(op, _)| self.state.get_bv_from_operand(op))
                    .collect::<anyhow::Result<Vec<_>>>()
                    .unwrap();

                // Create new location at the start of function to call, and store
                // our current position in the callstack so we can return here later.
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
            ReturnValue::Return(v) => Some(v),
            ReturnValue::Void => None,
            ReturnValue::Throw(_) => todo!("support throw in calls"),
            ReturnValue::Abort => return Err(VMError::Abort),
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
        trace!("{}", instr);
        todo!()
    }

    fn landingpad(&mut self, instr: &instruction::LandingPad) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn catchpad(&mut self, instr: &instruction::CatchPad) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }

    fn cleanuppad(&mut self, instr: &instruction::CleanupPad) -> Result<()> {
        trace!("{}", instr);
        todo!()
    }
}

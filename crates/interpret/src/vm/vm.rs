use either::Either;
use llvm_ir::instruction::{BinaryOp, HasResult, InlineAssembly, UnaryOp};
use llvm_ir::{
    instruction, terminator, Constant, Function, Instruction, IntPredicate, Name, Operand,
    Terminator, Type,
};
use log::{debug, trace, warn};

use crate::hooks::{FnInfo, Hooks};
use crate::llvm::{
    const_to_bv, get_constant_int_value, get_inner_type, get_offset_in_bits, operand_to_bv,
    size_in_bits, size_in_bytes,
};
use crate::project::Project;
use crate::solver::{BinaryOperation, Solver, BV};
use crate::vm::location::Location;
use crate::vm::{Call, Callsite, Path, State, VMError};
use crate::vm::{ExecutionError, Result};

#[derive(Debug)]
pub enum ReturnValue {
    /// The function returns a `BV` value.
    Return(BV),

    /// The function returns void.
    Void,

    /// The function throws.
    Throw(BV),

    /// The function aborted.
    Abort,
}

// Might be what I actually want.
//
// So Throw & Abort are errors which are caught. And then before returning
// to the user it could be transformed into another one.
//
// So the issue with the ReturnValue enum is that for, at least, abort we want
// to basically return back to the caller. Going up our current callstack
// (and the interpreted program's callstack).
//
// TODO: Check throw how that is supposed to work. If I use an error for that
// it might mess up when they have to be caught and stuff. Esp since variables
// are scoped the to interpreteted callstack.
pub enum ReturnVal {
    Value(BV),

    Void,
}

pub struct VM<'a> {
    /// Project that the VM executes on.
    project: &'a Project,

    /// Current state the VM uses for its execution.
    pub state: State<'a>,

    backtracking_paths: Vec<Path<'a>>,

    pub solver: Solver,

    hooks: Hooks,
}

impl<'a> Iterator for VM<'a> {
    type Item = Result<ReturnValue>;

    fn next(&mut self) -> Option<Self::Item> {
        trace!("Executing next path");
        Some(self.backtrack_and_continue())
    }
}

impl<'a> VM<'a> {
    /// Creates a new VM for a project that starts in the given function.
    ///
    /// A new VM has one path set up. To start execution either call `run` or use the VM as an
    /// iterator.
    pub fn new(fn_name: &str, project: &'a Project) -> Result<Self> {
        debug!("Creating VM, starting at function {}", fn_name);

        let (func, module) = project.fn_by_name(fn_name)?;
        let solver = Solver::new();

        let state = State::new(project, module, func, solver.clone());

        let mut vm = VM {
            state: state.clone(), // Dummy state.
            project,
            backtracking_paths: Vec::new(),
            solver,
            hooks: Hooks::new(),
        };

        vm.state.vars.enter_scope();
        vm.setup_parameters()?;

        let path = Path::new(vm.state.clone());
        vm.backtracking_paths.push(path);

        Ok(vm)
    }

    /// Starts executing the VM.
    pub fn run(&mut self) -> Result<ReturnValue> {
        // let r = self.execute_to_terminator()?;
        self.backtrack_and_continue()
    }

    pub fn run_all(&mut self) -> Result<()> {
        let mut paths_explored = 0;
        while self.backtracking_paths.len() > 0 {
            paths_explored += 1;
            trace!(
                "---------- RUN_ALL: Paths: {}",
                self.backtracking_paths.len()
            );
            let mut r = self.backtrack_and_continue();
            if let Err(VMError::Execution(ExecutionError::Abort)) = r {
                r = Ok(ReturnValue::Abort);
            }
            let r = r?;
            println!("R: {:?}", r);
        }
        println!("Explored {} paths", paths_explored);
        Ok(())
    }

    fn setup_parameters(&mut self) -> Result<()> {
        for param in self.state.current_loc.func.parameters.iter() {
            let size = size_in_bits(&param.ty, self.project).unwrap();
            assert_ne!(size, 0);

            let bv = self.solver.bv(size as u32);
            self.state.vars.insert(param.name.clone(), bv).unwrap();
        }

        Ok(())
    }

    /// Start executing from the current location.
    ///
    /// However when the execution stops we check if the execution state's callstack is empty, if
    /// not we have resumed execution inside a called function.
    ///
    /// This means we have to return this value to the previous callstack location and continue
    /// execution there.
    fn execute(&mut self) -> Result<ReturnValue> {
        loop {
            let result = self.execute_to_terminator()?;
            let mut callsite = if let Some(callstack) = self.state.callstack.pop() {
                callstack
            } else {
                // If we don't have any callstacks to pop, we're done. So return the result to the
                // caller and let them explore more paths if they want.
                return Ok(result);
            };

            match result {
                ReturnValue::Return(ret_val) => {
                    let dst_name = match callsite.instruction {
                        super::Call::Call(instr) => instr.dest.clone(),
                        super::Call::Invoke(instr) => Some(instr.result.clone()),
                    };

                    // Set our destination value, if it has a name.
                    if let Some(name) = dst_name {
                        self.state.assign_bv(name, ret_val).unwrap();
                    }
                }
                ReturnValue::Void => {}
                ReturnValue::Throw(_) => panic!("Throws are not handled yet"),

                // If we hit an abort, abort this as well.
                ReturnValue::Abort => return Ok(result),
            }

            // For `Call` we go to the next instruction, and for `Invoke` we enter the label that
            // it specifies.
            if matches!(callsite.instruction, Call::Call(_)) {
                callsite.location.inc_pc();
            } else if matches!(callsite.instruction, Call::Invoke(_)) {
                todo!()
            }

            self.state.current_loc = callsite.location;
        }
    }

    fn execute_to_terminator(&mut self) -> Result<ReturnValue> {
        debug!(
            "execute_to_terminator: block {} in function {}",
            self.state.current_loc.block.name, self.state.current_loc.func.name
        );

        let offset_into_block = self.state.current_loc.get_instruction_offset();
        let mut has_recored_path_entry = false;
        for (pc, inst) in self
            .state
            .current_loc
            .block
            .instrs
            .iter()
            .enumerate()
            .skip(offset_into_block)
        {
            self.state.current_loc.set_location(inst, pc);
            if !has_recored_path_entry {
                has_recored_path_entry = true;
                // self.state.record_path_entry();
            }

            let result = match &inst {
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
            };

            match result {
                Ok(_) => {} // No errors.
                // Check if unsats should be squashed.
                Err(e) => return Err(e),
            }
        }

        let terminator = &self.state.current_loc.block.term;
        self.state.current_loc.set_terminator(terminator);

        // If we didn't execute any instructions earlier, still record the path
        // entry.
        if !has_recored_path_entry {
            // self.state.record_path_entry();
        }

        // Check callbacks.

        // Handle terminator.
        let r = match terminator {
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
        }?;
        println!("a Returning {:?}", r);

        Ok(r)
    }

    fn save_backtracking_path(&mut self, bb_label: &Name, constraint: Option<BV>) -> Result<()> {
        trace!(
            "Save backtracking path: bb_label={:?}, constraint={:?}",
            bb_label,
            constraint
        );

        // Create a new context-level, this is so the backtracking point has
        // a valid solver when we resume execution (I think).
        self.solver.push(1);

        // Location where we resume the execution at.
        let jump_location = Location::jump_bb(self.state.current_loc.clone(), bb_label).unwrap();

        let path = Path::new_with_constraint(self.state.clone(), jump_location, constraint);
        self.backtracking_paths.push(path);

        Ok(())
    }

    fn backtrack_and_continue(&mut self) -> Result<ReturnValue> {
        if let Some(path) = self.backtracking_paths.pop() {
            trace!(
                "Backtrack to {:?}, {} paths remain",
                path,
                self.backtracking_paths.len()
            );

            // Replace the current state.
            self.state = path.state;

            // Add the contraint.
            if let Some(constraint) = path.constraint {
                // Return to the the solver context when the path was created.
                self.solver.pop(1);

                constraint.assert();
            }

            // Resume execution.
            self.execute()
        } else {
            panic!("no more paths available");
        }
    }

    fn call_fn(&mut self, f: &'a Function, arguments: Vec<BV>) -> Result<ReturnValue> {
        if arguments.len() != f.parameters.len() {
            if f.is_var_arg {
                panic!("variadic functions are not supported");
            } else {
                panic!("invalid fn call");
            }
        }

        // Create a new variable scope for the function we're about to call.
        self.state.vars.enter_scope();

        // Map arguments to parameters.
        for (param, arg) in f.parameters.iter().zip(arguments) {
            self.state.vars.insert(param.name.clone(), arg).unwrap();
        }

        // Update our current location and start executing the the new
        // function's basic block.
        let ret_val = self.execute_to_terminator()?;

        Ok(ret_val)
    }

    fn assign(&mut self, dst: &impl HasResult, src_bv: BV) -> Result<()> {
        let dst_ty = self.state.type_of(dst);
        let dst_sz = size_in_bits(&dst_ty, self.project).unwrap();

        println!(
            "bitcast ty: {:?}, size: {:?}, bv_size: {}",
            dst_ty,
            dst_sz,
            src_bv.get_width()
        );
        assert_eq!(dst_sz, src_bv.get_width() as usize);

        let dst_name = dst.get_result().clone();
        self.state.assign_bv(dst_name, src_bv).unwrap();
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Terminator Instructions
    // -------------------------------------------------------------------------

    fn ret(&mut self, instr: &terminator::Ret) -> Result<ReturnValue> {
        trace!("{}", instr);
        let ret_val = if let Some(operand) = &instr.return_operand {
            let value = operand_to_bv(operand, &mut self.state).unwrap();
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
        self.state.current_loc.set_basic_block(&instr.dest);
        self.execute_to_terminator()
    }

    fn condbr(&mut self, instr: &terminator::CondBr) -> Result<ReturnValue> {
        trace!("{}", instr);

        let cond = operand_to_bv(&instr.condition, &mut self.state).unwrap();
        let true_possible = self.solver.is_sat_with_constraint(&cond).unwrap();
        let false_possible = self
            .solver
            .is_sat_with_constraint(&cond.not().into())
            .unwrap();

        let jump = match (true_possible, false_possible) {
            (true, true) => {
                trace!("condbr: true and false possible");
                // Explore true first, then backtrack to false.
                self.save_backtracking_path(&instr.false_dest, Some(cond.not().into()))?;

                cond.assert();
                //self.state.current_loc.set_basic_block(&instr.true_dest);
                //self.execute_to_terminator()
                &instr.true_dest
            }
            (true, false) => {
                trace!("condbr: true possible");
                //cond.assert();
                //self.state.current_loc.set_basic_block(&instr.true_dest);
                //self.execute_to_terminator()
                &instr.true_dest
            }
            (false, true) => {
                trace!("condbr: false possible");
                //cond.not().assert();
                //self.state.current_loc.set_basic_block(&instr.false_dest);
                //self.execute_to_terminator()
                &instr.false_dest
            }
            (false, false) => {
                trace!("condbr: neither possible");
                //self.backtrack_and_continue()

                panic!("no paths");

                // return Err(anyhow!("no paths"));
            }
        };

        self.state.current_loc.set_basic_block(jump);
        self.execute_to_terminator()
    }

    fn switch(&mut self, instr: &terminator::Switch) -> Result<ReturnValue> {
        trace!("{}", instr);

        let mut paths_added = 0;
        let value = operand_to_bv(&instr.operand, &mut self.state).unwrap();

        // A switch can pick any paths, the default path can be chosen if value
        // can be something other than any of the switches.
        let mut default_cond = self.solver.bv_from_bool(true);

        // Check if any of the non-default cases can be reached.
        for (val, target) in instr.dests.iter() {
            let val = const_to_bv(val, &mut self.state).unwrap();

            // Build default condition.
            default_cond = default_cond.and(&value._ne(&val)).into();

            let cond = value._eq(&val);
            if self.solver.is_sat_with_constraint(&cond).unwrap() {
                trace!("switch: path {} possible", target);
                self.save_backtracking_path(target, Some(cond))?;
                paths_added += 1;
            }
        }

        // Check if the default case can be reached.
        if self.solver.is_sat_with_constraint(&default_cond).unwrap() {
            paths_added += 1;
            trace!("switch: default path possible");
            self.save_backtracking_path(&instr.default_dest, Some(default_cond))?;
        }

        // Backtrack immediately if we added a path, since the current path
        // is not done yet.
        if paths_added > 0 {
            self.backtrack_and_continue()
        } else {
            panic!("no paths in switch");
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

    fn unreachable(&mut self, instr: &terminator::Unreachable) -> Result<ReturnValue> {
        trace!("{}", instr);
        todo!()
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

        let lhs_bv = operand_to_bv(lhs, &mut self.state).unwrap();
        let rhs_bv = operand_to_bv(rhs, &mut self.state).unwrap();

        let result_bv = match op {
            BinaryOperation::Add => lhs_bv.add(&rhs_bv),
            BinaryOperation::Sub => lhs_bv.sub(&rhs_bv),
            BinaryOperation::Mul => lhs_bv.mul(&rhs_bv).into(),
            BinaryOperation::UDiv => lhs_bv.udiv(&rhs_bv).into(),
            BinaryOperation::SDiv => lhs_bv.sdiv(&rhs_bv).into(),
            BinaryOperation::URem => lhs_bv.urem(&rhs_bv).into(),
            BinaryOperation::SRem => lhs_bv.srem(&rhs_bv).into(),
        };

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

        let value = operand_to_bv(&instr.aggregate, &mut self.state).unwrap();

        let mut base_ty = self.state.type_of(&instr.aggregate);
        let mut total_offset = 0;
        for index in instr.indices.iter() {
            let (offset, ty) = get_offset_in_bits(&base_ty, *index as usize, &self.project);
            base_ty = ty;
            total_offset += offset;
        }

        let offset_high = total_offset + size_in_bits(&base_ty, &self.project).unwrap();
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

        let num_elements = get_constant_int_value(&instr.num_elements).unwrap();
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

        let addr = operand_to_bv(&instr.address, &mut self.state).unwrap();
        let dst_ty = self.state.type_of(instr);
        let dst_size = size_in_bits(&dst_ty, &self.project).unwrap() as u32;
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

        let value = operand_to_bv(&instr.value, &mut self.state).unwrap();
        let addr = operand_to_bv(&instr.address, &mut self.state).unwrap();
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
        let base_addr = operand_to_bv(&instr.address, &mut self.state).unwrap();

        // The offsets modifies the address ptr, and this is the type of what
        // is currently pointed to.
        let mut curr_ty = self.state.type_of(&instr.address);
        for index in instr.indices.iter() {
            dbg!(curr_ty.as_ref());

            let is_struct = matches!(
                curr_ty.as_ref(),
                Type::NamedStructType { .. } | Type::StructType { .. }
            );

            let (offset, ty) = if is_struct {
                // Concrete indexing into a struct.
                let index = get_constant_int_value(index).unwrap();
                let (offset, ty) = get_offset_in_bits(&curr_ty, index, &self.project);

                let offset_bytes = (offset / 8) as u64;
                let offset = self.solver.bv_from_u64(offset_bytes, base_addr.get_width());

                (offset, ty)
            } else {
                // Symbolic index. We cannot support struct indexing here, since
                // we calculate the offset as size of type * index, which won't
                // offset correctly for structs.
                let index = operand_to_bv(index, &mut self.state).unwrap();
                let index = index.zero_ext(base_addr.get_width());

                let bytes = size_in_bytes(&curr_ty, &self.project).unwrap().unwrap() as u64;
                let bytes = self.solver.bv_from_u64(bytes, base_addr.get_width());

                let ty = get_inner_type(&curr_ty, &self.project).unwrap();

                let offset = bytes.mul(&index).into();
                (offset, ty)
            };

            base_addr.add(&offset);
            curr_ty = ty;
        }

        dbg!(&instr);

        self.state
            .assign_bv(instr.get_result().clone(), base_addr)
            .unwrap();
        Ok(())
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
                let value = operand_to_bv(&instr.operand, &mut self.state).unwrap();
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
                let value = operand_to_bv(&instr.operand, &mut self.state).unwrap();
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
        let src_bv = operand_to_bv(instr.get_operand(), &mut self.state).unwrap();
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

        let lhs = operand_to_bv(&instr.operand0, &mut self.state).unwrap();
        let rhs = operand_to_bv(&instr.operand1, &mut self.state).unwrap();
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
        self.state
            .vars
            .insert(instr.get_result().clone(), result)
            .unwrap();
        Ok(())
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

        let ret_val = if let Some(hook) = self.hooks.get(&fn_name) {
            let info = FnInfo {
                arguments: instr.arguments.clone(),
                return_attrs: instr.return_attributes.clone(),
                fn_attrs: instr.function_attributes.clone(),
            };
            let res = hook(self, info).unwrap();
            res
        } else {
            let (function, module) = self.project.fn_by_name(&fn_name)?;
            println!("-- FN FOUND");

            let arguments = instr
                .arguments
                .iter()
                .map(|(op, _)| {
                    println!("arg {:?}", op);
                    operand_to_bv(op, &mut self.state)
                })
                .collect::<anyhow::Result<Vec<_>>>()
                .unwrap();
            println!("-- ARGS DONE");

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
        };

        // Assign return value
        let ret_val = match ret_val {
            ReturnValue::Return(v) => Some(v),
            ReturnValue::Void => None,
            ReturnValue::Throw(_) => todo!("support throw in calls"),
            ReturnValue::Abort => return Err(ExecutionError::Abort.into()),
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

    // ---------------------------------------------------------------------------------------------

    fn resolve_function(&mut self, function: &Either<InlineAssembly, Operand>) -> Result<String> {
        trace!("resolve fn: {:?}", function);
        match function {
            Either::Left(_) => todo!(),
            Either::Right(operand) => match operand {
                Operand::ConstantOperand(constant) => match constant.as_ref() {
                    Constant::GlobalReference {
                        name: Name::Name(name),
                        ..
                    } => {
                        println!("global ref name: {:?}", name);
                        Ok(name.to_string())
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            },
        }
    }

    // fn resolve_function(&mut self, function: &Either<InlineAssembly, Operand>) -> Result<()> {
    //     let name = match function {
    //         Either::Left(f) => {
    //             todo!()
    //         }
    //         Either::Right(operand) => {
    //             // Check if this is a global reference to a function.
    //             match operand {
    //                 Operand::ConstantOperand(constant) => match constant.as_ref() {
    //                     Constant::GlobalReference { name, .. } => name.to_string(),
    //                     _ => {
    //                         todo!() // nothing
    //                     }
    //                 },
    //                 _ => {
    //                     todo!() // nothing
    //                 }
    //             };

    //             let bv = operand_to_bv(operand, &mut self.state)?;
    //             let mut fn_ptr = self.solver.get_solutions_for_bv(&bv, 1)?;
    //             let fn_ptr = match &mut fn_ptr {
    //                 Solutions::None => Err(anyhow!("unsat")),
    //                 Solutions::Exactly(v) => Ok(v.pop().unwrap()),
    //                 Solutions::AtLeast(_) => Err(anyhow!("multiple possible fn ptrs")),
    //             }?;

    //             todo!()
    //         }
    //     };

    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(path: &str, f: &str) -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();

        let project = Project::from_bc_path(path).expect("Failed to create project");
        let mut vm = VM::new(f, &project).expect("Failed to create VM");
        let result = vm.run_all();

        println!("res: {:?}", result);
        result
    }

    #[test]
    fn vm_simple() {
        assert!(run("tests/bcs/simple.bc", "main").is_ok());
    }

    #[test]
    fn vm_ifs() {
        assert!(run("tests/bcs/ifs.bc", "main").is_ok());
    }

    #[test]
    fn vm_multiple_paths() {
        assert!(run("tests/bcs/multiple_paths.bc", "foo").is_ok());
    }

    #[test]
    fn vm_call() {
        assert!(run("tests/bcs/call.bc", "bar").is_ok());
    }

    #[test]
    fn vm_enum_match() {
        assert!(run("tests/bcs/match.bc", "match::main").is_ok());
    }

    #[test]
    fn vm_get_array_checked() {
        assert!(run("tests/bcs/oob.bc", "oob::get").is_ok());
    }

    #[test]
    fn vm_get_array_unchecked() {
        assert!(run("tests/bcs/oob-unchecked.bc", "oob_unchecked::get").is_ok());
    }

    #[test]
    fn vm_catch_panic() {
        assert!(run("tests/bcs/catch_panic.bc", "catch_panic::foo").is_ok());
    }
}

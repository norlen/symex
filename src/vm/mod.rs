use either::Either;
use llvm_ir::{
    instruction::{HasResult, InlineAssembly},
    Constant, Function, Name, Operand,
};
use log::{debug, trace};

use crate::{
    common::SolutionVariable,
    project::{ModuleHandle, Project},
    solver::{Solutions, Solver, BV},
};

mod error;
mod globals;
mod instructions;
mod state;

pub use error::{Result, VMError};
pub use globals::*;
pub use state::*;

#[derive(Debug, PartialEq, Eq)]
pub enum ReturnValue {
    Value(BV),

    Void,
}

pub enum TerminatorResult {
    Return(Option<BV>),

    /// If execution should be resumed. This can happen for terminators such as `br`, where the next
    /// label should be executed.
    Branch,
}

pub struct VM<'a> {
    /// Project that the VM executes on.
    pub project: &'a Project,

    /// Current state the VM uses for its execution.
    pub state: State<'a>,

    backtracking_paths: Vec<Path<'a>>,

    pub solver: Solver,

    /// Parameters passed to the initial entry function.
    pub parameters: Vec<SolutionVariable>,
}

impl<'a> Clone for VM<'a> {
    fn clone(&self) -> Self {
        Self {
            project: self.project,
            state: self.state.clone(),
            backtracking_paths: self.backtracking_paths.clone(),
            solver: self.solver.duplicate(),
            parameters: self.parameters.clone(),
        }
    }
}

impl<'a> Iterator for VM<'a> {
    type Item = Result<ReturnValue>;

    fn next(&mut self) -> Option<Self::Item> {
        self.backtrack_and_resume_execution()
    }
}

impl<'a> VM<'a> {
    /// Creates a new VM for a project that starts in the given function.
    ///
    /// A new VM has one path set up. To start execution either call `run` or use the VM as an
    /// iterator.
    pub fn new(fn_name: &str, project: &'a Project) -> Result<Self> {
        debug!("Creating VM, starting at function {}", fn_name);

        let (module, function) = project.find_entry_function(fn_name)?;
        let solver = Solver::new();
        let state = State::new(project, module, function, solver.clone());

        let mut vm = VM {
            state,
            project,
            backtracking_paths: Vec::new(),
            solver,
            parameters: Vec::new(),
        };

        // Setup before the execution of a function can start.
        vm.state.vars.enter_scope();
        // Bind parameters to unknown values.
        vm.setup_parameters()?;

        // Create a backtracking point to the start of the function.
        let bb_label = &vm.state.current_loc.block.name;
        debug!("bb_label {}", bb_label);

        vm.save_backtracking_path(bb_label, None)?;

        Ok(vm)
    }

    // Helper to create unconstrained symbols for all parameters.
    fn setup_parameters(&mut self) -> Result<()> {
        for param in self.state.current_loc.func.parameters.iter() {
            let size = self.project.bit_size(&param.ty)?;
            assert_ne!(size, 0);

            let bv = self.solver.bv(size as u32, &param.name.to_string());
            let solution_var = SolutionVariable {
                name: param.name.to_string(),
                value: bv.clone(),
                ty: Some(param.ty.clone()),
            };
            self.parameters.push(solution_var);

            self.state.vars.insert(param.name.clone(), bv)?;
        }

        Ok(())
    }

    /// Execute a single path in the VM to completion.
    pub fn run(&mut self) -> Option<Result<ReturnValue>> {
        self.backtrack_and_resume_execution()
    }

    /// Resume execution from a stored path.
    ///
    /// When we restore the state from a stored path, the VM's call stack is empty. So it cannot
    /// use the VM's stack to know when to stop execution.
    ///
    /// `resume_execution` instead iteratively executes until all the callsites in the stored state
    /// have been exhausted.
    fn resume_execution(&mut self) -> Result<ReturnValue> {
        loop {
            // When executing a basic block we can either get a value from e.g. `ret` but we can
            // also want to resume execution for e.g. `br`.
            let result = self.execute_function()?;

            let mut callsite = if let Some(callstack) = self.state.callstack.pop() {
                callstack
            } else {
                // If we don't have any callstacks to pop, we're done. So return the result to the
                // caller and let them explore more paths if they want.
                return Ok(result);
            };

            if let ReturnValue::Value(result) = result {
                // Get the callee's variable that should be set with the result of the call.
                let callee_target = match callsite.instruction {
                    Call::Call(instr) => instr.dest.clone(),
                    Call::Invoke(instr) => Some(instr.result.clone()),
                };

                // Set the destination variable if it exists.
                if let Some(name) = callee_target {
                    self.state.assign_bv(name, result)?;
                }
            }

            // Set up which instruction to execute next.
            match callsite.instruction {
                // For `Call` the next instruction should be executed.
                Call::Call(_) => {
                    callsite.location.inc_pc();
                    self.state.current_loc = callsite.location;
                }

                // For `invoke` since this is a successful case, jump to the label it specifies.
                Call::Invoke(instr) => {
                    // Restore the callsite's location.
                    std::mem::swap(&mut self.state.current_loc, &mut callsite.location);

                    self.branch(&instr.return_label)?;
                }
            }
        }
    }

    /// Execute a single function.
    ///
    /// This will iteratively go through each basic block until it hits a terminator that returns
    /// a value (or void).
    fn execute_function(&mut self) -> Result<ReturnValue> {
        loop {
            let result = self.execute_basic_block()?;
            match result {
                // For function returns, the value is just returned.
                TerminatorResult::Return(value) => {
                    let value = match value {
                        Some(value) => ReturnValue::Value(value),
                        None => ReturnValue::Void,
                    };
                    return Ok(value);
                }

                // Continue as long as we hit branches in the IR.
                TerminatorResult::Branch => {}
            }
        }
    }

    /// Execute a single basic block.
    ///
    /// Resumes execution from the instruction stored in the current location and runs until it
    /// hits a terminator. This can either be a value, or a variant denoting a branch has occurred
    /// and that the callee should call this function again to resume execution in that basic block.
    fn execute_basic_block(&mut self) -> Result<TerminatorResult> {
        debug!(
            "function {}, block: {}",
            self.state.current_loc.func.name, self.state.current_loc.block.name
        );

        let offset_into_block = self.state.current_loc.get_instruction_offset();
        for (pc, inst) in self
            .state
            .current_loc
            .block
            .instrs
            .iter()
            .enumerate()
            .skip(offset_into_block)
        {
            self.state.current_loc.set_location(pc);
            self.process_instruction(inst)?;
        }

        // Handle terminator.
        let terminator = &self.state.current_loc.block.term;
        self.state.current_loc.set_terminated(terminator);
        self.process_terminator(terminator)
    }

    /// Save a backtracking path that can be resumed later.
    pub fn save_backtracking_path(
        &mut self,
        bb_label: &Name,
        constraint: Option<BV>,
    ) -> Result<()> {
        trace!(
            "Save backtracking path: bb_label={:?}, constraint={:?}",
            bb_label,
            constraint
        );

        // Create a new context-level, this is so the backtracking point has
        // a valid solver when we resume execution (I think).
        self.solver.push();

        // Location where we resume the execution at.
        let jump_location = Location::jump_bb(self.state.current_loc.clone(), bb_label).unwrap();

        let path = Path::new_with_constraint(self.state.clone(), jump_location, constraint);
        self.backtracking_paths.push(path);

        Ok(())
    }

    /// Backtrack and resume execution with that state.
    fn backtrack_and_resume_execution(&mut self) -> Option<Result<ReturnValue>> {
        if let Some(path) = self.backtracking_paths.pop() {
            trace!("Backtrack, {} paths remain", self.backtracking_paths.len());

            // Replace the current state.
            self.state = path.state;

            // Return to the the solver context when the path was created.
            self.solver.pop();

            // Add the constraint.
            if let Some(constraint) = path.constraint {
                self.solver.assert(&constraint);
            }

            // Resume execution.
            Some(self.resume_execution())
        } else {
            None
        }
    }

    /// Helper to call a function.
    ///
    /// This will store the current location as a callsite, and set up the target location. It will
    /// then execute all the basic blocks in the function and return the final return value of the
    /// function.
    pub fn call_fn(
        &mut self,
        call: Call<'a>,
        module: ModuleHandle,
        function: &'a Function,
        arguments: Vec<BV>,
    ) -> Result<ReturnValue> {
        if arguments.len() != function.parameters.len() {
            if function.is_var_arg {
                panic!("variadic functions are not supported");
            } else {
                panic!("invalid fn call");
            }
        }

        // Create new location at the start of function to call, and store our current
        // position in the callstack so we can return here later.
        let mut new_location = Location::new(module, function);
        std::mem::swap(&mut new_location, &mut self.state.current_loc);

        let callsite = match call {
            Call::Call(call) => Callsite::from_call(new_location, call),
            Call::Invoke(invoke) => Callsite::from_invoke(new_location, invoke),
        };
        // let callsite = Callsite::from_invoke(new_location, instr);
        self.state.callstack.push(callsite);

        // Create a new variable scope for the function we're about to call.
        self.state.vars.enter_scope();

        // Map arguments to parameters.
        for (param, arg) in function.parameters.iter().zip(arguments) {
            self.state.vars.insert(param.name.clone(), arg)?;
        }

        // Update our current location and start executing the the new function's basic block.
        //
        // Don't really have to care about errors, since if an error occurs the path is dead.
        let return_value = self.execute_function()?;

        // Restore callsite.
        let callsite = self.state.callstack.pop().unwrap();
        self.state.current_loc = callsite.location;

        Ok(return_value)
    }

    /// Helper to update the location to another basic block inside the same function.
    pub fn branch(&mut self, target: &Name) -> Result<TerminatorResult> {
        self.state.current_loc.set_basic_block(target);
        Ok(TerminatorResult::Branch)
    }

    /// Helper function to assign to the result variable in the instruction.
    pub fn assign(&mut self, dst: &impl HasResult, src_bv: BV) -> Result<()> {
        let target_ty = self.state.type_of(dst);
        let target_size = self.project.bit_size(&target_ty)?;
        assert_eq!(target_size, src_bv.len());

        self.state.vars.insert(dst.get_result().clone(), src_bv)
    }

    // ---------------------------------------------------------------------------------------------

    pub fn resolve_function(
        &mut self,
        function: &Either<InlineAssembly, Operand>,
    ) -> Result<String> {
        match function {
            Either::Left(_) => todo!(),
            Either::Right(operand) => match operand {
                Operand::ConstantOperand(constant) => match constant.as_ref() {
                    Constant::GlobalReference {
                        name: Name::Name(name),
                        ..
                    } => Ok(name.to_string()),
                    _ => todo!(),
                },
                Operand::LocalOperand { .. } => {
                    let addr = self.state.get_var(operand)?;
                    let solutions = self.solver.get_solutions_for_bv(&addr, 1).unwrap();
                    //dbg!(&solutions);
                    match solutions {
                        Solutions::None => todo!(),
                        Solutions::Exactly(v) => {
                            let addr = v[0].as_u64().unwrap();
                            let f = self
                                .state
                                .global_references
                                .get_function_from_address(addr, self.state.current_loc.module)
                                .unwrap();
                            Ok(f.to_string())
                        }
                        Solutions::AtLeast(_) => todo!(),
                    }
                }
                Operand::MetadataOperand => todo!(),
            },
        }
    }
}

use either::Either;
use llvm_ir::{
    instruction::{HasResult, InlineAssembly},
    Constant, Function, Name, Operand,
};
use log::{debug, trace};

use crate::{
    project::Project,
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

pub struct VM<'a> {
    /// Project that the VM executes on.
    pub project: &'a Project,

    /// Current state the VM uses for its execution.
    pub state: State<'a>,

    backtracking_paths: Vec<Path<'a>>,

    pub solver: Solver,

    /// Parameters passed to the initial entry function.
    pub parameters: Vec<BV>,
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
        self.backtrack_and_continue()
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
            self.parameters.push(bv.clone());

            self.state.vars.insert(param.name.clone(), bv)?;
        }

        Ok(())
    }

    /// Execute a single path in the VM to completion.
    pub fn run(&mut self) -> Option<Result<ReturnValue>> {
        self.backtrack_and_continue()
    }

    /// Helper to run all the paths the VM finds.
    pub fn run_all(&mut self) -> Vec<Result<ReturnValue>> {
        let mut results = Vec::new();

        let mut paths_explored = 0;
        while let Some(path_result) = self.backtrack_and_continue() {
            paths_explored += 1;
            results.push(path_result);
        }
        println!("Explored {} paths", paths_explored);
        results
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

            // For `Call` we go to the next instruction, and for `Invoke` we enter the label that
            // it specifies.
            if matches!(callsite.instruction, Call::Call(_)) {
                callsite.location.inc_pc();
            } else if matches!(callsite.instruction, Call::Invoke(_)) {
                return Err(VMError::UnsupportedInstruction);
            }

            self.state.current_loc = callsite.location;
        }
    }

    /// Starts execution from the current instruction stored in the state, and runs until it hits
    /// a terminator instruction at the end of the basic block.
    fn execute_to_terminator(&mut self) -> Result<ReturnValue> {
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

    pub fn branch(&mut self, target: &Name) -> Result<ReturnValue> {
        self.state.current_loc.set_basic_block(target);
        self.execute_to_terminator()
    }

    fn backtrack_and_continue(&mut self) -> Option<Result<ReturnValue>> {
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
            Some(self.execute())
        } else {
            None
        }
    }

    pub fn call_fn(&mut self, f: &'a Function, arguments: Vec<BV>) -> Result<ReturnValue> {
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
            self.state.vars.insert(param.name.clone(), arg)?;
        }

        // Update our current location and start executing the the new
        // function's basic block.
        let ret_val = self.execute_to_terminator()?;

        Ok(ret_val)
    }

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
        //trace!("\n\nresolve fn: {:?}", function);
        match function {
            Either::Left(_) => todo!(),
            Either::Right(operand) => match operand {
                Operand::ConstantOperand(constant) => match constant.as_ref() {
                    Constant::GlobalReference {
                        name: Name::Name(name),
                        ..
                    } => {
                        // println!("global ref name: {:?}", name);
                        Ok(name.to_string())
                    }
                    _ => todo!(),
                },
                Operand::LocalOperand { .. } => {
                    let addr = self.state.get_var(operand)?;
                    let solutions = self.solver.get_solutions_for_bv(&addr, 1).unwrap();
                    dbg!(&solutions);
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

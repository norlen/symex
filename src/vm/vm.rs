use either::Either;
use llvm_ir::instruction::{HasResult, InlineAssembly};
use llvm_ir::{Constant, Function, Name, Operand};
use log::{debug, trace};

use crate::{
    project::Project,
    solver::{Solutions, Solver, BV},
    vm::{AllocationType, Call, Location, Path, Result, State},
};

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

    pub(super) solver: Solver,
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

        let (function, module) = project.find_entry_function(fn_name)?;
        let solver = Solver::new();
        let state = State::new(project, module, function, solver.clone());

        let mut vm = VM {
            state,
            project,
            backtracking_paths: Vec::new(),
            solver,
        };

        // Setup before exeuction of function can start.
        vm.state.vars.enter_scope();
        vm.setup_parameters()?;

        // Create a backtracking point to the start of the function.
        let bb_label = &vm.state.current_loc.block.name;
        vm.save_backtracking_path(bb_label, None)?;

        Ok(vm)
    }

    // Helper to create unconstrained symbols for all parameters.
    fn setup_parameters(&mut self) -> Result<()> {
        for param in self.state.current_loc.func.parameters.iter() {
            let size = self.project.bit_size(&param.ty)?;
            assert_ne!(size, 0);

            let bv = self.solver.bv(size as u32);
            self.state.vars.insert(param.name.clone(), bv)?;
        }

        Ok(())
    }

    /// Starts executing the VM.
    pub fn run(&mut self) -> Result<ReturnValue> {
        // let r = self.execute_to_terminator()?;
        self.backtrack_and_continue()
    }

    // Helper to run all the paths the VM finds.
    pub fn run_all(&mut self) -> Vec<Result<ReturnValue>> {
        let mut results = Vec::new();

        let mut paths_explored = 0;
        while !self.backtracking_paths.is_empty() {
            paths_explored += 1;
            trace!(
                "---------- RUN_ALL: Paths: {}",
                self.backtracking_paths.len()
            );

            let r = self.backtrack_and_continue();

            // println!("Result: {:?}", r);
            results.push(r);
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

            match result {
                ReturnValue::Value(ret_val) => {
                    let dst_name = match callsite.instruction {
                        super::Call::Call(instr) => instr.dest.clone(),
                        super::Call::Invoke(instr) => Some(instr.result.clone()),
                    };

                    // Set our destination value, if it has a name.
                    if let Some(name) = dst_name {
                        self.state.assign_bv(name, ret_val)?;
                    }
                }
                ReturnValue::Void => {}
                // ReturnValue::Throw(_) => panic!("Throws are not handled yet"),

                // If we hit an abort, abort this as well.
                // ReturnValue::Abort => return Ok(result),
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
            let result = self.process_instruction(inst);

            match result {
                Ok(_) => {} // No errors.
                // Check if unsats should be squashed.
                Err(e) => return Err(e),
            }
        }

        let terminator = &self.state.current_loc.block.term;
        self.state.current_loc.set_terminator(terminator);

        // Handle terminator.
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
        self.solver.push(1);

        // Location where we resume the execution at.
        let jump_location = Location::jump_bb(self.state.current_loc.clone(), bb_label).unwrap();

        let path = Path::new_with_constraint(self.state.clone(), jump_location, constraint);
        self.backtracking_paths.push(path);

        Ok(())
    }

    // TODO: Ideally, we want to separate the two cases:
    // 1. When we actually backtrack and run stuff.
    // 2. Just for convenience. In that we can add any numbers of paths, then
    //    just execute the last one we added. Without it actually "backtracking".
    //    So the difference lies in how we handle *our* stack. In backtracking
    //    we can assume the callstacks does not match. However, in our regular
    //    execution we can assume this.

    pub fn branch(&mut self, target: &Name) -> Result<ReturnValue> {
        self.state.current_loc.set_basic_block(target);
        self.execute_to_terminator()
    }

    fn backtrack_and_continue(&mut self) -> Result<ReturnValue> {
        if let Some(path) = self.backtracking_paths.pop() {
            trace!("Backtrack, {} paths remain", self.backtracking_paths.len());

            // Replace the current state.
            self.state = path.state;

            // Return to the the solver context when the path was created.
            self.solver.pop(1);

            // Add the contraint.
            if let Some(constraint) = path.constraint {
                self.solver.assert(&constraint);
            }

            // Resume execution.
            self.execute()
        } else {
            panic!("no more paths available");
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
        trace!("\n\nresolve fn: {:?}", function);
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
                                .globals
                                .get_func(addr, self.state.current_loc.module)
                                .unwrap();
                            match &f.kind {
                                AllocationType::Variable(_) => todo!(),
                                AllocationType::Function(f) => Ok(f.name.to_string()),
                            }
                        }
                        Solutions::AtLeast(_) => todo!(),
                    }
                }
                Operand::MetadataOperand => todo!(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VMError;

    fn run(path: &str, f: &str) -> Vec<Result<ReturnValue>> {
        let _ = env_logger::builder().is_test(true).try_init();

        let project = Project::from_bc_path(path).expect("Failed to create project");
        let mut vm = VM::new(f, &project).expect("Failed to create VM");
        let result = vm.run_all();

        println!("res: {:?}", result);
        result
    }

    #[test]
    fn vm_simple() {
        let res = run("tests/bcs/simple.bc", "main");
        assert!(res[0].is_ok());
    }

    #[test]
    fn vm_ifs() {
        let res = run("tests/bcs/ifs.bc", "main");
        assert!(res.len() > 0);
        res.iter().for_each(|res| assert!(res.is_ok()));
    }

    #[test]
    fn vm_multiple_paths() {
        let res = run("tests/bcs/multiple_paths.bc", "foo");
        assert!(res.len() > 0);
        res.iter().for_each(|res| assert!(res.is_ok()));
    }

    #[test]
    fn vm_call() {
        let res = run("tests/bcs/call.bc", "bar");
        assert!(res.len() > 0);
        res.iter().for_each(|res| assert!(res.is_ok()));
    }

    #[test]
    fn vm_enum_match() {
        let res = run("tests/bcs/match.bc", "match::main");
        assert!(res.len() > 0);
        res.iter().for_each(|res| assert!(res.is_ok()));
    }

    #[test]
    fn vm_get_array_checked1() {
        let res = run("tests/bcs/oob.bc", "oob::get");
        assert!(res.len() > 0);
        assert!(res[0].is_ok()); // actually out of bounds
        assert_eq!(res[1], Err(VMError::Abort(-1)));
    }

    #[test]
    fn vm_get_array_checked2() {
        let res = run("tests/bcs/oob.bc", "oob::get2");
        assert!(res.len() > 0);
        assert!(matches!(res[0], Ok(ReturnValue::Value(_))));
        assert_eq!(res[1], Err(VMError::Abort(-1)));
    }

    #[test]
    fn vm_get_array_unchecked() {
        let res = run("tests/bcs/oob-unchecked.bc", "oob_unchecked::get");
        assert!(res[0].is_ok()); // actually out of bounds
    }

    // #[test]
    // fn vm_test_locals() {
    //     let res = run("tests/bc2/locals.bc", "locals::app::foo");
    //     assert_eq!(res[0], Err(VMError::OutOfBounds));
    // }

    #[test]
    fn vm_test_traits() {
        let res = run("tests/bcs/traits.bc", "traits::foo");
        assert!(res[0].is_ok());
    }

    #[test]
    fn vm_test_out_of_bounds_checked() {
        let res = run(
            "tests/samples/out_of_bounds.bc",
            "out_of_bounds::out_of_bounds",
        );
        assert!(matches!(res[0], Ok(ReturnValue::Value(_))));
        assert_eq!(res[1], Err(VMError::Abort(-1)));
    }

    // #[test]
    // fn vm_test_out_of_bounds_unchecked() {
    //     let res = run(
    //         "tests/samples/out_of_bounds.bc",
    //         "out_of_bounds::out_of_bounds_unchecked",
    //     );
    //     assert_eq!(res[0], Err(VMError::OutOfBounds));
    // }

    // #[test]
    // fn vm_test_vec() {
    //     let res = run("tests/bcs/vec.bc", "vec::foo");
    //     assert_eq!(res[0], Err(VMError::OutOfBounds));
    // }

    #[test]
    fn vm_test_globals() {
        let res = run("tests/samples/globals.bc", "globals::foo");
        assert!(matches!(res[0], Ok(ReturnValue::Value(_))));
        assert!(matches!(res[1], Ok(ReturnValue::Value(_))));
    }
}

use either::Either;
use llvm_ir::{
    instruction::{HasResult, InlineAssembly},
    Constant, Function, Name, Operand,
};
use tracing::trace;

use crate::{
    core::smt::{Expression, Solutions, Solver},
    executor::{
        llvm::{
            project::Project, InstructionIndex, LLVMExecutorError, LLVMInstruction, LLVMState,
            Location, ModuleHandle, Result, StackFrame,
        },
        vm::VM,
    },
    path_exploration::Path,
    smt::DExpr,
    PathExploration,
};

#[derive(Debug, Clone)]
pub enum TerminatorResult {
    Return(Option<DExpr>),

    /// If execution should be resumed. This can happen for terminators such as `br`, where the next
    /// label should be executed.
    Branch,
}

// TODO: Rename this and add more variants.
//
// Should exist support for:
// - Suspend this path, to be resumed (think BFS)
// - Probably abort(0) as well, for success cases
// - Should probably change value to option, with None being void
// - ...

#[derive(Debug, PartialEq, Eq)]
pub enum ReturnValue {
    Value(DExpr),

    Void,
}

/// Think of this as a "thread" for each state. We basically want to run the states.
///
/// So create one of these for every state, and run that
/// for n states, we can spawn n of these if we want then
///
///
pub struct LLVMExecutor<'vm> {
    pub vm: &'vm mut VM,

    pub state: LLVMState,

    pub project: &'static Project,
}

// impl<'vm> Executor for LLVMExecutor<'vm> {}

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
    pub fn resume_execution(&mut self) -> Result<ReturnValue> {
        loop {
            // When executing a basic block we can either get a value from e.g. `ret` but we can
            // also want to resume execution for e.g. `br`.
            let result = self.execute_function()?;

            // If we returned to the top-level function, i.e. the initial stack frame then execution
            // is done. Keep the stack frame intact so variables can be checked.
            if self.state.stack_frames.len() == 1 {
                return Ok(result);
            }

            // Remove stack frame from where we returned from.
            self.state.stack_frames.pop().unwrap();

            let location = &self.state.stack_frames.last().unwrap().location;
            match location.instr {
                InstructionIndex::NotStarted => panic!(),
                InstructionIndex::Instruction(offset) => {
                    let call = match &location.block.instrs[offset] {
                        llvm_ir::Instruction::Call(c) => c,
                        _ => panic!("not a call"),
                    };

                    if let ReturnValue::Value(e) = result {
                        if let Some(name) = &call.dest {
                            self.assign_register(name.clone(), e).unwrap();
                        } else {
                            panic!("no target for assignemnet");
                        }
                    }

                    self.state
                        .stack_frames
                        .last_mut()
                        .unwrap()
                        .location
                        .inc_pc();
                }
                InstructionIndex::Terminated => {
                    let term = match &location.block.term {
                        llvm_ir::Terminator::Invoke(i) => i,
                        _ => panic!("not an invoke"),
                    };

                    if let ReturnValue::Value(e) = result {
                        self.assign_result(term, e).unwrap();
                    }

                    self.branch(&term.return_label)?;
                }
            };
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
        // TODO: This is a bit messy, but cannot store a reference to the stack frame since that
        // will trigger multiple borrows from self.
        let offset_into_block = self
            .state
            .stack_frames
            .last_mut()
            .unwrap()
            .location
            .get_instruction_offset();

        for (pc, instr) in self
            .state
            .stack_frames
            .last_mut()
            .unwrap()
            .location
            .block
            .instrs
            .iter()
            .enumerate()
            .skip(offset_into_block)
        {
            self.state
                .stack_frames
                .last_mut()
                .unwrap()
                .location
                .set_location(pc);

            LLVMInstruction::process_instruction(self, instr)?;
        }

        // Handle terminator.
        let terminator = &self
            .state
            .stack_frames
            .last_mut()
            .unwrap()
            .location
            .block
            .term;
        self.state
            .stack_frames
            .last_mut()
            .unwrap()
            .location
            .set_terminated(terminator);

        LLVMInstruction::process_terminator(self, terminator)
    }

    /// Helper to call a function.
    ///
    /// This will store the current location as a callsite, and set up the target location. It will
    /// then execute all the basic blocks in the function and return the final return value of the
    /// function.
    pub fn call_fn(
        &mut self,
        module: ModuleHandle,
        function: &'static Function,
        arguments: Vec<DExpr>,
    ) -> Result<ReturnValue> {
        if arguments.len() != function.parameters.len() {
            if function.is_var_arg {
                panic!("variadic functions are not supported");
            } else {
                panic!("invalid fn call");
            }
        }

        if self.state.stack_frames.len() >= self.vm.cfg.max_call_depth {
            panic!("Call depth exceeded");
        }

        // Create new stack frame for our branch location.
        let new_location = Location::new(module, function);
        let mut stack_frame = StackFrame::new(new_location);

        // Map arguments to parameters.
        for (param, arg) in function.parameters.iter().zip(arguments) {
            stack_frame.registers.insert(param.name.clone(), arg);
        }

        // Push the newly created stack frame, execution will resume from this location.
        //
        // We can return early on errors without having to clear the stack frames since once a path
        // encounters an error the path will never be resumed again.
        self.state.stack_frames.push(stack_frame);
        let return_value = self.execute_function()?;
        self.state.stack_frames.pop();

        Ok(return_value)
    }

    /// Helper to update the location to another basic block inside the same function.
    pub fn branch(&mut self, target: &Name) -> Result<TerminatorResult> {
        let stack_frame = self.state.stack_frames.last_mut().unwrap();

        let entry_count = stack_frame
            .basic_block_entry_count
            .entry(target.clone())
            .or_default();
        *entry_count += 1;

        if *entry_count >= self.vm.cfg.max_iter_count {
            panic!("Maximum loop bound exceeded");
        }

        stack_frame.location.set_basic_block(target);

        Ok(TerminatorResult::Branch)
    }

    /// Save a backtracking path that can be resumed later.
    pub fn save_path(&mut self, bb_label: &Name, constraint: Option<DExpr>) -> Result<()> {
        trace!(
            "Save backtracking path: bb_label={:?}, constraint={:?}",
            bb_label,
            constraint
        );

        let current_location = self.state.stack_frames.last().unwrap().location.clone();
        let jump_location = Location::jump_bb(current_location, bb_label)?;

        let forked_state = self.state.fork(jump_location);
        let path = Path::new(forked_state, constraint);

        self.vm.paths.save_path(path);

        Ok(())
    }

    pub fn fork(&mut self, constraint: DExpr) -> Result<()> {
        let forked_state = self.state.clone();
        let path = Path::new(forked_state, Some(constraint));

        self.vm.paths.save_path(path);
        Ok(())
    }

    pub fn assign_result<I>(&mut self, instr: &I, e: DExpr) -> Result<()>
    where
        I: HasResult,
    {
        self.assign_register(instr.get_result().clone(), e)
    }

    pub fn assign_register(&mut self, name: Name, e: DExpr) -> Result<()> {
        self.state
            .stack_frames
            .last_mut()
            .unwrap()
            .registers
            .insert(name, e);
        Ok(())
    }

    /// Resolves a call operand to a list of function names.
    pub fn resolve_function(
        &mut self,
        function: &Either<InlineAssembly, Operand>,
    ) -> Result<Vec<String>> {
        let operand = match function {
            Either::Left(_) => panic!("Inline assembly not supported yet"),
            Either::Right(operand) => operand,
        };

        // If the operand is a constant global reference to a named function, then just return that
        // directly.
        if let Operand::ConstantOperand(operand) = operand {
            if let Constant::GlobalReference {
                name: Name::Name(name),
                ..
            } = operand.as_ref()
            {
                return Ok(vec![name.to_string()]);
            }
        }

        // TODO: May just want to concretize here and fork, the resuming operation at this location.
        // changing to only return a single function name.

        // Concretize the jump address.
        let addr = self.state.get_expr(operand)?;
        let solutions = self
            .state
            .constraints
            .get_values(&addr, self.vm.cfg.max_fn_ptr_resolutions)?;

        // Return an error if the maximum amount of address resolutions is exceeded, otherwise
        // for state for each address.
        let addresses = match solutions {
            Solutions::AtLeast(_) => panic!("Too many solutions for function pointer"),
            Solutions::Exactly(s) => s,
        };

        let mut function_names = Vec::new();
        for addr in addresses {
            // Should always fit, only 32-bit and 64-bit should be supported.
            let addr = addr.get_constant().unwrap();

            let current_module = self.state.stack_frames.last().unwrap().location.module;

            let name = self
                .state
                .global_references
                .get_function_from_address(addr, current_module)
                .ok_or_else(|| {
                    LLVMExecutorError::FunctionNotFound(format!("Function with address: {addr:x}"))
                })?;

            function_names.push(name.to_owned());
        }
        Ok(function_names)
    }
}

// fn create_concrete_value(binary_str: &str, ty: &Type, project: &Project) -> ConcreteValue {
//     let ty = create_concrete_value_type(binary_str, ty, project);
//     ConcreteValue {
//         raw: binary_str.to_owned(),
//         ty,
//     }
// }

// fn create_concrete_value_type(binary_str: &str, ty: &Type, project: &Project) -> ConcreteValueType {
//     let n = binary_str.len();

//     use Type::*;
//     match ty {
//         IntegerType { bits } => ConcreteValueType::Value {
//             value: u128::from_str_radix(binary_str, 2).unwrap(),
//             bits: *bits,
//         },

//         PointerType { .. } => ConcreteValueType::Value {
//             value: u128::from_str_radix(binary_str, 2).unwrap(),
//             bits: project.ptr_size,
//         },

//         VectorType {
//             element_type,
//             num_elements,
//             ..
//         }
//         | ArrayType {
//             element_type,
//             num_elements,
//         } => {
//             let el_size = project.bit_size(element_type).unwrap() as usize;

//             let mut elements = Vec::new();
//             for i in 0..*num_elements {
//                 let high = n - i * el_size;
//                 let low = n - (i + 1) * el_size;
//                 let s = &binary_str[low..high];
//                 let element = create_concrete_value(s, element_type, project);
//                 elements.push(element);
//             }

//             ConcreteValueType::Array(elements)
//         }

//         StructType { element_types, .. } => {
//             let mut fields = Vec::new();
//             let mut current_offset = 0;
//             for el_ty in element_types {
//                 let size = project.bit_size(el_ty).unwrap() as usize;
//                 let low = n - (current_offset + size);
//                 let high = n - current_offset;
//                 let s = &binary_str[low..high];

//                 fields.push(create_concrete_value(s, el_ty, project));
//                 current_offset += size;
//             }

//             ConcreteValueType::Struct(fields)
//         }

//         NamedStructType { name } => match project.get_named_struct(name) {
//             Some(named_struct) => match named_struct {
//                 NamedStructDef::Opaque => todo!(),
//                 NamedStructDef::Defined(ty) => create_concrete_value_type(binary_str, ty, project),
//             },
//             None => todo!(),
//         },

//         // TODO
//         FPType(_) => todo!(),
//         FuncType { .. } => todo!(),
//         VoidType => todo!(),
//         X86_MMXType => todo!(),
//         X86_AMXType => todo!(),
//         MetadataType => todo!(),
//         LabelType => todo!(),
//         TokenType => todo!(),
//     }
// }

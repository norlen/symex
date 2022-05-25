use crate::executor::llvm::project::Project;
use crate::executor::vm::Path;
use crate::executor::vm::VM;
use crate::smt::DExpr;
use crate::smt::Expression;
use crate::smt::Solutions;
use crate::smt::Solver;
use either::Either;
use llvm_ir::instruction::HasResult;
use llvm_ir::instruction::InlineAssembly;
use llvm_ir::Constant;
use llvm_ir::Function;
use llvm_ir::Name;
use llvm_ir::Operand;
use tracing::trace;

use super::InstructionIndex;
use super::LLVMExecutorError;
use super::ModuleHandle;
use super::StackFrame;
use super::{LLVMInstruction, LLVMState, Location, Result};

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

            // let stack_frame = match self.state.stack_frames.pop() {
            //     Some(stack_frame) => stack_frame,
            //     None => {
            //         // If we don't have any callstacks to pop, we're done. So return the result to the
            //         // caller and let them explore more paths if they want.
            //         // return Ok(result);
            //         return Ok(());
            //     }
            // };
            if self.state.stack_frames.len() == 1 {
                return Ok(result);
                // let result = match result {
                //     ReturnValue::Value(v) => {
                //         let v = self.state.constraints.get_value(&v).unwrap();
                //         let s = v.to_binary_string();
                //         let terminator =
                //             &self.state.stack_frames.last().unwrap().location.block.term;

                //         let ty = match terminator {
                //             Terminator::Ret(instr) => match &instr.return_operand {
                //                 Some(op) => Some(self.state.type_of(op)),
                //                 None => None,
                //             },
                //             _ => None,
                //         };
                //         let value = match ty {
                //             Some(ty) => {
                //                 let size = self.project.bit_size(ty.as_ref())?;
                //                 let s = format!("{s:0>size$}", size = size as usize);
                //                 Some(create_concrete_value(&s, ty.as_ref(), self.project))
                //             }
                //             None => None,
                //         };
                //         crate::executor::vm::ReturnValue::Value(value)
                //     }
                //     ReturnValue::Void => crate::executor::vm::ReturnValue::Void,
                // };
                // return Ok(result);
            }
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
        // let stack_frame = self.state.stack_frames.last_mut().unwrap();
        // let location = &mut stack_frame.location;

        // debug!(
        //     "function {}, block: {}",
        //     location.func.name, location.block.name
        // );

        // TODO: fix this shit. Cannot borrow from self multiple times, is the biggest issue with
        // storing a ref to the location.

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

        // Create new location at the start of function to call, and store our current
        // position in the callstack so we can return here later.
        // let mut new_location = Location::new(module, function);
        // std::mem::swap(&mut new_location, &mut self.state.current_loc);

        // let callsite = match call {
        //     Call::Call(call) => Callsite::from_call(new_location, call),
        //     Call::Invoke(invoke) => Callsite::from_invoke(new_location, invoke),
        // };
        let new_location = Location::new(module, function);
        let mut stack_frame = StackFrame::new(new_location);

        // Map arguments to parameters.
        for (param, arg) in function.parameters.iter().zip(arguments) {
            stack_frame.registers.insert(param.name.clone(), arg);
        }

        // self.state.callstack.push(callsite);

        // Create a new variable scope for the function we're about to call.
        // self.state.vars.enter_scope();

        self.state.stack_frames.push(stack_frame);

        // Update our current location and start executing the the new function's basic block.
        //
        // Don't really have to care about errors, since if an error occurs the path is dead.
        let return_value = self.execute_function()?;

        // Restore callsite.
        // let callsite = self.state.callstack.pop().unwrap();
        // self.state.current_loc = callsite.location;

        self.state.stack_frames.pop();

        Ok(return_value)
    }

    /// Helper to update the location to another basic block inside the same function.
    pub fn branch(&mut self, target: &Name) -> Result<TerminatorResult> {
        // self.state.current_loc.set_basic_block(target);
        self.state
            .stack_frames
            .last_mut()
            .unwrap()
            .location
            .set_basic_block(target);

        Ok(TerminatorResult::Branch)
    }

    /// Save a backtracking path that can be resumed later.
    pub fn save_path(&mut self, bb_label: &Name, constraint: Option<DExpr>) -> Result<()> {
        trace!(
            "Save backtracking path: bb_label={:?}, constraint={:?}",
            bb_label,
            constraint
        );

        // Create a new context-level, this is so the backtracking point has
        // a valid solver when we resume execution (I think).
        // self.state.solver.push();

        // Location where we resume the execution at.
        // let jump_location = Location::jump_bb(self.state.current_loc.clone(), bb_label).unwrap();

        // let path = Path::new_with_constraint(self.state.clone(), jump_location, constraint);
        // self.backtracking_paths.push(path);

        let jump_location = Location::jump_bb(
            self.state.stack_frames.last().unwrap().location.clone(),
            bb_label,
        )
        .unwrap();
        let forked_state = self.state.fork(jump_location);
        let path = Path::new(forked_state, constraint);

        self.vm.save_path(path);

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

    // pub fn type_of<T: Typed>(&self, t: &T) -> TypeRef {
    //     // self.project.type_of(t, self.current_loc.module)
    //     todo!()
    // }

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

        let addr = match operand {
            Operand::LocalOperand { .. } => self.state.get_expr(operand)?,
            Operand::ConstantOperand(_) => self.state.get_expr(operand)?,
            Operand::MetadataOperand => return Err(LLVMExecutorError::MalformedInstruction),
        };

        let solutions = self.state.constraints.get_values(&addr, 5)?;
        let addresses = match solutions {
            // This may be a bug, in that the pointer is unconstrained.
            // however, todo and fork state
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

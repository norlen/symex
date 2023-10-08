use std::collections::{HashMap, HashSet};

use llvm_ir::{
    constant::{Constant, Expression},
    instruction::LLVMIntPredicate,
    instruction::{BasicBlock, Instruction},
    Function, Global, GlobalVariable, Value,
};
use tracing::{debug, trace, warn};

use super::{binop, bit_size, project::Project};
use crate::vm::{executor::convert_to_map, LLVMExecutorError};
use crate::{
    memory::ObjectMemory,
    smt::{DContext, DExpr, DSolver},
    util::Variable,
    vm::Result,
};

/// Stack frame keeps track of information related to a specific stack frame.
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// The [`Function`] that is being executed.
    function: Function,

    /// Variables created in a specific stack frame.
    registers: HashMap<Value, DExpr>,

    location: Location,
}

impl StackFrame {
    pub fn new(function: Function) -> Result<Self> {
        let basic_block = function
            .first_basic_block()
            .expect("Function has no basic blocks");

        Ok(Self {
            function,
            registers: HashMap::new(),
            location: Location::new(basic_block)?,
        })
    }

    pub fn new_from_function(function: Function, arguments: &[DExpr]) -> Result<Self> {
        debug!(
            "Creating new stack frame for function: {:?} with arguments: {arguments:?}",
            function.name()
        );
        let mut registers = HashMap::new();
        for (value, expr) in function.parameters().zip(arguments) {
            debug!("Argument: {value} -> {expr:?}");
            registers.insert(value, expr.clone());
        }

        let basic_block = function
            .first_basic_block()
            .expect("Function has no basic blocks");
        Ok(Self {
            function,
            registers,
            location: Location::new(basic_block)?,
        })
    }

    pub fn function(&self) -> &Function {
        &self.function
    }

    pub fn set_register(&mut self, register: Value, expr: DExpr) {
        self.registers.insert(register, expr);
    }

    pub fn get_register(&self, register: &Value) -> Option<&DExpr> {
        self.registers.get(register)
    }

    /// Changes the location to another basic block.
    pub fn set_basic_block(&mut self, bb: BasicBlock) -> Result<()> {
        self.location = Location::new_jump(self.location.clone(), bb)?;
        Ok(())
    }

    pub fn current_instruction(&self) -> Option<&Instruction> {
        self.location.instr.as_ref()
    }

    pub fn previous_block(&self) -> Option<&BasicBlock> {
        self.location.previous_block.as_ref()
    }

    /// Increase the current instruction index by one.
    pub fn increase_pc(&mut self) {
        self.location.increase_pc();
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    /// The current [`BasicBlock`] that is being executed.
    pub block: BasicBlock,

    /// The [`BasicBlock`] that was executed prior to the current basic block.
    pub previous_block: Option<BasicBlock>,

    /// Determines the current instruction to be executed in in the [`BasicBlock`].
    pub instr: Option<Instruction>,
}

impl Location {
    pub fn new(block: BasicBlock) -> Result<Self> {
        let instr = block
            .first_instruction()
            .expect("Basic block has no instructions");

        Ok(Self {
            block,
            previous_block: None,
            instr: Some(instr),
        })
    }

    pub fn new_jump(current: Location, new_block: BasicBlock) -> Result<Self> {
        let instr = new_block
            .first_instruction()
            .expect("Basic block has no instructions");

        Ok(Self {
            block: new_block,
            previous_block: Some(current.block),
            instr: Some(instr),
        })
    }

    /// Increase the current instruction index by one.
    pub fn increase_pc(&mut self) {
        if let Some(instr) = self.instr.take() {
            self.instr = self.block.next_instruction(instr);
        }
    }
}

#[derive(Clone)]
pub struct LLVMState {
    // Check if I should have this here, or maybe just pass the executor instead
    pub project: &'static Project,

    /// SMT Context.
    pub ctx: &'static DContext,

    /// The path condition, holds all the saved constraints.
    pub constraints: DSolver,

    /// List of variables marked as symbolic.
    pub marked_symbolic: Vec<Variable>,

    pub memory: ObjectMemory,

    pub stack_frames: Vec<StackFrame>,

    // /// Global references, these can be either a [Function] or a [GlobalVariable].
    // ///
    // /// This holds the mapping between the name of the global reference and its address.
    // pub global_references: GlobalReferences,
    pub global_lookup_rev: HashMap<u64, Value>,
    pub global_lookup: HashMap<Value, u64>,
    pub init_global: HashSet<u64>,
}

impl std::fmt::Debug for LLVMState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LLVMState")
            .field("constraints", &self.constraints)
            .field("marked_symbolic", &self.marked_symbolic)
            .field("memory", &self.memory)
            .field("stack_frames", &self.stack_frames)
            .finish()
    }
}

impl LLVMState {
    pub fn new(
        ctx: &'static DContext,
        project: &'static Project,
        constraints: DSolver,
        function: Function,
    ) -> Result<Self> {
        let memory = ObjectMemory::new(ctx, project.ptr_size, constraints.clone());

        let stack_frame = StackFrame::new(function)?;
        Ok(Self {
            ctx,
            constraints,
            marked_symbolic: Vec::new(),
            memory,
            stack_frames: vec![stack_frame],
            project,
            global_lookup: HashMap::new(),
            global_lookup_rev: HashMap::new(),
            init_global: HashSet::new(),
        })
    }

    pub fn current_frame(&self) -> Result<&StackFrame> {
        self.stack_frames
            .last()
            .ok_or(LLVMExecutorError::NoStackFrame)
    }

    pub fn current_frame_mut(&mut self) -> Result<&mut StackFrame> {
        self.stack_frames
            .last_mut()
            .ok_or(LLVMExecutorError::NoStackFrame)
    }

    /// Retrieves or creates an [Expr] from an [Operand] or [Constant].
    pub fn get_expr(&mut self, value: &Value) -> Result<DExpr> {
        trace!("Get expression: {value:?} -> {value}");
        operand_to_expr(self, value)
    }
}

/// Convert an operand to an expression.
pub fn operand_to_expr(state: &mut LLVMState, value: &Value) -> Result<DExpr> {
    match value {
        Value::Function(_) | Value::Global(_) | Value::Constant(_) => const_to_expr(state, value),
        Value::Instruction(_) | Value::Argument(_) => state
            .current_frame_mut()?
            .get_register(&value)
            .cloned()
            .ok_or_else(|| LLVMExecutorError::LocalNotFound(format!("{value}"))),

        // Not supported.
        Value::InlineAsm | Value::Metadata => panic!("Unexpected operand"),
    }
}

pub(crate) fn init_gv(state: &mut LLVMState, gv: &GlobalVariable, addr: u64) {
    if state.init_global.contains(&addr) {
        return;
    }

    if let Some(initializer) = gv.initializer() {
        match state.get_expr(&initializer) {
            Ok(value) => {
                let addre = state.ctx.from_u64(addr, state.project.ptr_size);
                state.memory.write(&addre, value).unwrap();
                state.init_global.insert(addr);
            }
            Err(err) => {
                warn!("Error initializing global: {:?}", err);
            }
        }
    }
}

/// Convert a constant to an expression.
///
/// Requires the final size to not be zero sized. State is required since global references
/// are allowed in constants.
pub(crate) fn const_to_expr(state: &mut LLVMState, constant: &Value) -> Result<DExpr> {
    let value = const_to_expr_zero_size(state, constant)?;
    value.ok_or(LLVMExecutorError::UnexpectedZeroSize)
}

/// Convert a constant to an expression that allows the entire thing to be zero sized.
///
/// State is required since global references are allowed in constants.
pub(crate) fn const_to_expr_zero_size(
    state: &mut LLVMState,
    value: &Value,
) -> Result<Option<DExpr>> {
    let constant = match value {
        Value::Function(_) | Value::Global(_) => {
            let global_address = state.global_lookup.get(&value).cloned();
            debug!("gv lookup: {global_address:?}");
            let Some(global_address) = global_address else {
                panic!("Global not found");
            };

            if let Value::Global(Global::Variable(global_variable)) = value {
                init_gv(state, global_variable, global_address);
            }

            let expr = state.ctx.from_u64(global_address, state.project.ptr_size);
            return Ok(Some(expr));
        }

        // Handle later.
        Value::Constant(constant) => constant,

        // Not expected.
        Value::Instruction(_) | Value::Argument(_) | Value::Metadata | Value::InlineAsm => {
            panic!("Unexpected constant: {value:?}")
        }
    };

    let ty = constant.ty();
    let e = match constant {
        // `Undef` indicates that the value may have an unspecified bit pattern. It it allowed for
        // all types except `void` and `label`.
        //
        // These should indicate that the program is well behaved no matter the value, thus we put
        // these as unconstrained so we can check for possible errors.
        //
        // Not sure if the generated LLVM does not allow for these errors to happen, but if it does
        // those kind of errors are covered.
        Constant::Undef(_) | Constant::Poison(_) => {
            let size = bit_size(&ty, state.project.ptr_size)?;

            let e = match size {
                0 => None,
                n => {
                    let name = format!("undef_{}", rand::random::<u32>());
                    Some(state.ctx.unconstrained(n as u32, &name))
                }
            };
            Ok(e)
        }

        Constant::TargetNone(_) => todo!("TargetNone"),

        // Both null pointers and the aggregate of zeroes are initialized to zero.
        Constant::PointerNull(_) | Constant::AggregateZero(_) => {
            let size = bit_size(&ty, state.project.ptr_size)?;
            Ok(match size {
                0 => None,
                n => Some(state.ctx.zero(n as u32)),
            })
        }

        Constant::Integer(constant) => {
            let bits = bit_size(&ty, state.project.ptr_size)?;
            Ok(Some(state.ctx.from_u64(constant.value(), bits)))
        }

        Constant::Float(_) => todo!("const fp"),

        Constant::Array(array) => {
            let elements = array
                .elements()
                .filter_map(|constant| const_to_expr_zero_size(state, &constant).transpose())
                .collect::<Result<Vec<_>>>()?;

            Ok(elements.into_iter().reduce(|acc, v| v.concat(&acc)))
        }
        Constant::Vector(vector) => {
            let elements = vector
                .elements()
                .filter_map(|constant| const_to_expr_zero_size(state, &constant).transpose())
                .collect::<Result<Vec<_>>>()?;

            Ok(elements.into_iter().reduce(|acc, v| v.concat(&acc)))
        }

        Constant::Structure(structure) => {
            let elements = structure
                .fields()
                .filter_map(|constant| const_to_expr_zero_size(state, &constant).transpose())
                .collect::<Result<Vec<_>>>()?;

            Ok(elements.into_iter().reduce(|acc, v| v.concat(&acc)))
        }

        Constant::Expression(expression) => Some(match expression {
            Expression::Trunc(i) => {
                let operation = |value: DExpr, target_size: u32| value.slice(0, target_size - 1);
                convert_to_map(state, i.value(), &i.to_type(), operation)
            }
            Expression::ZExt(i) => {
                let operation = |value: DExpr, target_size: u32| value.zero_ext(target_size);
                convert_to_map(state, i.value(), &i.to_type(), operation)
            }
            Expression::SExt(i) => {
                let operation = |value: DExpr, target_size: u32| value.sign_ext(target_size);
                convert_to_map(state, i.value(), &i.to_type(), operation)
            }
            Expression::FPTrunc(_) => todo!(),
            Expression::FPExt(_) => todo!(),
            Expression::FPToUI(_) => todo!(),
            Expression::FPToSI(_) => todo!(),
            Expression::UIToFP(_) => todo!(),
            Expression::SIToFP(_) => todo!(),
            Expression::PtrToInt(i) => {
                let operation = |value: DExpr, target_size: u32| value.resize_unsigned(target_size);
                convert_to_map(state, i.value(), &i.to_type(), operation)
            }
            Expression::IntToPtr(i) => {
                let operation = |value: DExpr, target_size: u32| value.resize_unsigned(target_size);
                convert_to_map(state, i.value(), &i.to_type(), operation)
            }
            Expression::BitCast(i) => const_to_expr(state, &i.value()),
            Expression::AddrSpaceCast(i) => const_to_expr(state, &i.value()),
            Expression::GetElementPtr(_) => todo!(),
            Expression::ICmp(i) => {
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
                binop(state, &i.lhs(), &i.rhs(), f)
            }
            Expression::FCmp(_) => todo!(),
            Expression::ExtractElement(_) => todo!(),
            Expression::InsertElement(_) => todo!(),
            Expression::ShuffleVector(_) => todo!(),
            Expression::Add(i) => binop(state, &i.lhs(), &i.rhs(), DExpr::add),
            Expression::FAdd(_) => todo!(),
            Expression::Sub(i) => binop(state, &i.lhs(), &i.rhs(), DExpr::sub),
            Expression::FSub(_) => todo!(),
            Expression::Mul(i) => binop(state, &i.lhs(), &i.rhs(), DExpr::mul),
            Expression::FMul(_) => todo!(),
            Expression::Shl(i) => binop(state, &i.lhs(), &i.rhs(), DExpr::sll),
            Expression::LShr(i) => binop(state, &i.lhs(), &i.rhs(), DExpr::srl),
            Expression::AShr(i) => binop(state, &i.lhs(), &i.rhs(), DExpr::sra),
            Expression::Xor(i) => binop(state, &i.lhs(), &i.rhs(), DExpr::xor),
        })
        .transpose(),
    };

    e.map(|e| e.map(|e| e.simplify()))
}

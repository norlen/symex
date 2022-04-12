//! TODO
use anyhow::{anyhow, Result};
use llvm_ir::{BasicBlock, DebugLoc, Function, HasDebugLoc, Module, Name};

/// Determines the index to the next instruction to execute in a Basic Block.
#[derive(Debug, Clone)]
pub enum InstructionIndex {
    /// Default value, no instruction has been set yet. This will execute
    /// the first instruction (index 0). Used to differentiate between not
    /// started, and that the first instruction is executed.
    NotStarted,

    /// Index to the next instruction to execute.
    Instruction(usize),

    /// The basic block has terminated execution.
    Terminated,
}

/// Location references a place in the code.
///
/// References which module, function, basic block and instruction where
/// the execution is currently at.
#[derive(Clone)]
pub struct Location<'a> {
    /// The `Module` that is currently being executing in.
    pub module: &'a Module,

    /// The `Function` that is being executed.
    pub func: &'a Function,

    /// The current `BasicBlock` that is being executed.
    pub block: &'a BasicBlock,

    /// Determines the current instruction being executed in in the `BasicBlock`.
    pub instr: InstructionIndex,

    /// todo
    pub source_loc: Option<&'a DebugLoc>,
}

// The llvm-ir types do not implement Debug, so create a basic on that only
// has debug output for the types defined here.
impl<'a> std::fmt::Debug for Location<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Location").finish()
    }
}

impl<'a> Location<'a> {
    /// Create a new [`Location`].
    ///
    /// The specific location will be in the given module and function, and will
    /// pick the first basic block.
    pub fn new(module: &'a Module, func: &'a Function) -> Self {
        let block = func.basic_blocks.get(0).expect("failed to get bb");
        Self {
            module,
            func,
            block,
            instr: InstructionIndex::NotStarted,
            source_loc: None,
        }
    }

    /// Changes the location to another basic block, this also resets the
    /// [`InstructionIndex`] to `NotStarted`.
    pub fn jump_bb(location: Location<'a>, block_label: &Name) -> Result<Self> {
        let block = location
            .func
            .get_bb_by_name(block_label)
            .ok_or_else(|| anyhow!("Failed to find basic block {:?}", block_label))?;

        Ok(Self {
            module: location.module,
            func: location.func,
            block,
            instr: InstructionIndex::NotStarted,
            source_loc: None,
        })
    }

    /// Increase the current instruction index by one. The `NotStarted` variant
    /// is treated the same as `Instruction(0)`.
    ///
    /// # Panic
    ///
    /// Will panic if the [`Location`] has [`InstructionIndex::Terminated`].
    pub fn inc_pc(&mut self) {
        assert!(
            !matches!(self.instr, InstructionIndex::Terminated),
            "should not call inc_pc on a terminated location"
        );

        let current_pc = self.get_instruction_offset();
        if current_pc + 1 >= self.block.instrs.len() {
            self.instr = InstructionIndex::Terminated;
        } else {
            self.instr = InstructionIndex::Instruction(current_pc + 1);
            self.source_loc = self.block.instrs[current_pc + 1].get_debug_loc().as_ref();
        }
    }

    /// Returns the current instruction index into the basic block.
    ///
    /// This treats [`InstructionIndex::NotStarted`]
    pub fn get_instruction_offset(&self) -> usize {
        let num_instructions = self.block.instrs.len();
        match self.instr {
            InstructionIndex::NotStarted => 0,
            InstructionIndex::Instruction(i) => {
                assert!(i < num_instructions);
                i
            }
            InstructionIndex::Terminated => num_instructions,
        }
    }

    pub fn set_location(&mut self, pc: usize) {
        self.instr = InstructionIndex::Instruction(pc);
        self.source_loc = self.block.instrs[pc].get_debug_loc().as_ref();
    }

    pub fn set_terminator(&mut self, term: &'a impl HasDebugLoc) {
        self.instr = InstructionIndex::Terminated;
        self.source_loc = term.get_debug_loc().as_ref();
    }

    pub fn set_basic_block(&mut self, name: &Name) {
        self.block = self.func.get_bb_by_name(name).unwrap();
        self.instr = InstructionIndex::NotStarted;
    }
}

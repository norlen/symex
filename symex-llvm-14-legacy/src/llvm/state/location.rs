use llvm_ir::{BasicBlock, DebugLoc, Function, HasDebugLoc, Name};

use crate::llvm::{project::ModuleHandle, Result};

// TODO:
// - Remove `source_loc` and add function for that instead.
// - Add function for getting the current instruction. This should remove the need to keep a ref to
//   the function around e.g. in the previous stack frames.

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
pub struct Location {
    /// The `Module` that is currently being executing in.
    pub module: ModuleHandle,

    /// The `Function` that is being executed.
    pub func: &'static Function,

    /// The current `BasicBlock` that is being executed.
    pub block: &'static BasicBlock,

    /// The `BasicBlock` that was executed prior to the current basic block.
    pub previous_block: Option<&'static BasicBlock>,

    /// Determines the current instruction being executed in in the `BasicBlock`.
    pub instr: InstructionIndex,

    /// todo: remove and add function instead.
    pub source_loc: Option<&'static DebugLoc>,
    // todo: add function as well for getting current instruction.
}

// The llvm-ir types do not implement Debug, so create a basic on that only
// has debug output for the types defined here.
impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Location")
            .field("module", &self.module)
            .field("func", &self.func.name)
            .field("block", &self.block.name)
            .field("previous_block", &self.previous_block.map(|b| &b.name))
            .field("instr", &self.instr)
            .field("source_loc", &self.source_loc)
            .finish()
    }
}

impl Location {
    /// Create a new [`Location`].
    ///
    /// The specific location will be in the given module and function, and will
    /// pick the first basic block.
    pub fn new(module: ModuleHandle, func: &'static Function) -> Self {
        let block = func.basic_blocks.get(0).expect("failed to get bb");
        Self {
            module,
            func,
            block,
            previous_block: None,
            instr: InstructionIndex::NotStarted,
            source_loc: None,
        }
    }

    /// Changes the location to another basic block, this also resets the [`InstructionIndex`] to
    /// `NotStarted`.
    pub fn jump_bb(location: Location, block_label: &Name) -> Result<Self> {
        let block = location
            .func
            .get_bb_by_name(block_label)
            .expect("Failed to find basic block");
        // .ok_or_else(|| anyhow!("Failed to find basic block {:?}", block_label))?;

        Ok(Self {
            module: location.module,
            func: location.func,
            previous_block: Some(location.block),
            block,
            instr: InstructionIndex::NotStarted,
            source_loc: None,
        })
    }

    /// Increase the current instruction index by one. The `NotStarted` variant is treated the same
    /// as `Instruction(0)`.
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

    pub fn set_terminated(&mut self, term: &'static impl HasDebugLoc) {
        self.instr = InstructionIndex::Terminated;
        self.source_loc = term.get_debug_loc().as_ref();
    }

    pub fn set_basic_block(&mut self, name: &Name) {
        self.previous_block = Some(self.block);
        self.block = self.func.get_bb_by_name(name).unwrap();
        self.instr = InstructionIndex::NotStarted;
    }
}

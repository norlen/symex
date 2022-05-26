use crate::{llvm::LLVMState, DExpr};

mod dfs;

pub use dfs::DFSPathExploration;

/// A `Path` represents a single path of execution through a program. The path is composed by the
/// current execution state (`State`) and an optional constraint that will be asserted when this
/// path begins executing.
///
/// A single path may produce multiple other paths when encountering branching paths of execution.
#[derive(Debug, Clone)]
pub struct Path {
    /// The state to use when resuming execution.
    ///
    /// The location in the state should be where to resume execution at.
    pub state: LLVMState,

    /// Constraints to add before starting execution on this path.
    pub constraints: Vec<DExpr>,
}

impl Path {
    pub fn new(state: LLVMState, constraint: Option<DExpr>) -> Self {
        let constraints = match constraint {
            Some(c) => vec![c],
            None => vec![],
        };

        Self { state, constraints }
    }
}

pub trait PathExploration {
    fn save_path(&mut self, path: Path);

    fn get_path(&mut self) -> Option<Path>;
}

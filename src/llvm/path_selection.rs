//! Path exploration strategies.
//!
//! Currently the only supported strategy is [`DFSPathExploration`] which explores all paths using
//! depth-first search.
use crate::{
    core::path_selection::{self, PathSelection},
    llvm::state::LLVMState,
    smt::DExpr,
};

type Path = path_selection::Path<LLVMState, DExpr>;

/// Depth-first search path exploration.
///
/// Each path is explored for as long as possible, when a path finishes the most recently added
/// path is the next to be run.
#[derive(Debug, Clone)]
pub struct DFSPathSelection {
    paths: Vec<Path>,
}

impl DFSPathSelection {
    /// Creates new without any stored paths.
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }
}

impl PathSelection<LLVMState, DExpr> for DFSPathSelection {
    fn save_path(&mut self, path: Path) {
        path.state.constraints.push();
        self.paths.push(path);
    }

    fn get_path(&mut self) -> Option<Path> {
        match self.paths.pop() {
            Some(path) => {
                path.state.constraints.pop();
                Some(path)
            }
            None => None,
        }
    }
}

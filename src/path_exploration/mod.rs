//! Path exploration strategies.
//!
//! Currently the only supported strategy is [`DFSPathExploration`] which explores all paths using
//! depth-first search.
use crate::{
    core::path_exploration::{self, PathExploration},
    llvm::state::LLVMState,
    smt::DExpr,
};

type Path = path_exploration::Path<LLVMState, DExpr>;

/// Depth-first search path exploration.
///
/// Each path is explored for as long as possible, when a path finishes the most recently added
/// path is the next to be run.
#[derive(Debug, Clone)]
pub struct DFSPathExploration {
    paths: Vec<Path>,
}

impl DFSPathExploration {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }
}

impl PathExploration<LLVMState, DExpr> for DFSPathExploration {
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

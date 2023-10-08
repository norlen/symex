//! Path exploration strategies.
//!
//! Currently the only supported strategy is [`DFSPathExploration`] which explores all paths using
//! depth-first search.
use crate::smt::DExpr;

use super::state::LLVMState;

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
    /// Creates a new path starting at a certain state, optionally asserting a condition on the
    /// created path.
    pub fn new(state: LLVMState, constraint: Option<DExpr>) -> Self {
        let constraints = match constraint {
            Some(c) => vec![c],
            None => vec![],
        };

        Self { state, constraints }
    }
}

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

    /// Add a new path to be explored.
    pub fn save_path(&mut self, path: Path) {
        path.state.constraints.push();
        self.paths.push(path);
    }

    /// Retrieve the next path to explore.
    pub fn get_path(&mut self) -> Option<Path> {
        match self.paths.pop() {
            Some(path) => {
                path.state.constraints.pop();
                Some(path)
            }
            None => None,
        }
    }
}

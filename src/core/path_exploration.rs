//!
//!

/// A `Path` represents a single path of execution through a program. The path is composed by the
/// current execution state (`State`) and an optional constraint that will be asserted when this
/// path begins executing.
///
/// A single path may produce multiple other paths when encountering branching paths of execution.
#[derive(Debug, Clone)]
pub struct Path<State, E> {
    /// The state to use when resuming execution.
    ///
    /// The location in the state should be where to resume execution at.
    pub state: State,

    /// Constraints to add before starting execution on this path.
    pub constraints: Vec<E>,
}

impl<State, E> Path<State, E> {
    pub fn new(state: State, constraint: Option<E>) -> Self {
        let constraints = match constraint {
            Some(c) => vec![c],
            None => vec![],
        };

        Self { state, constraints }
    }
}

/// Path exploration strategy trait.
pub trait PathExploration<State, E> {
    fn save_path(&mut self, path: Path<State, E>);

    fn get_path(&mut self) -> Option<Path<State, E>>;
}

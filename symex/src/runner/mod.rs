/// Determine for which types of paths the solver should be invoked on.
#[derive(Debug)]
pub enum SolveFor {
    /// All paths.
    All,

    /// Paths which return errors. Currently this is both internal executor errors and program errors.
    Error,

    /// Paths which are sucessful.
    Success,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunConfig {
    /// Which paths should the solver be invoked on.
    pub solve_for: SolveFor,

    /// If concretized inputs should be shown.
    pub solve_inputs: bool,

    /// If concretized values should be displayed for variables passed to `symbolic`.
    pub solve_symbolics: bool,

    /// If concretized output values should be shown.
    pub solve_output: bool,
}

impl RunConfig {
    /// Determine if the solver should be invoked this specific result.
    ///
    /// Returns true of all paths should be solved, or if the result variant matches the given
    /// `SolveFor`.
    fn should_solve<T, E>(&self, result: &Result<T, E>) -> bool {
        use SolveFor::*;
        match self.solve_for {
            All => true,
            Error => matches!(result, Err(_)),
            Success => matches!(result, Ok(_)),
        }
    }
}

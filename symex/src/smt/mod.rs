use std::fmt::Debug;

pub mod smt_boolector;

pub type DExpr = smt_boolector::BoolectorExpr;
pub type DSolver = smt_boolector::BoolectorIncrementalSolver;
pub type DContext = smt_boolector::BoolectorSolverContext;
pub type DArray = smt_boolector::BoolectorArray;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SolverError {
    /// The set of constraints added to the solution are unsatisfiable.
    #[error("Unsat")]
    Unsat,

    /// Unknown error passed along from the SMT solver used.
    #[error("Unknown")]
    Unknown,

    /// Exceeded the passed maximum number of solutions.
    #[error("Exceeded number of solutions")]
    TooManySolutions,
}

#[derive(Debug)]
pub enum Solutions<E> {
    Exactly(Vec<E>),
    AtLeast(Vec<E>),
}

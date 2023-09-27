#[cfg(feature = "boolector")]
pub mod smt_boolector;

#[cfg(feature = "z3")]
pub mod smt_z3;

#[cfg(feature = "boolector")]
pub type DExpr = smt_boolector::BoolectorExpr;
#[cfg(feature = "boolector")]
pub type DSolver = smt_boolector::BoolectorIncrementalSolver;
#[cfg(feature = "boolector")]
pub type DContext = smt_boolector::BoolectorSolverContext;
#[cfg(feature = "boolector")]
pub type DArray = smt_boolector::BoolectorArray;

#[cfg(feature = "z3")]
pub type DExpr = smt_z3::Z3Expr<'static>;
#[cfg(feature = "z3")]
pub type DSolver = smt_z3::Z3SolverIncremental<'static>;
#[cfg(feature = "z3")]
pub type DContext = smt_z3::Z3SolverContext<'static>;
#[cfg(feature = "z3")]
pub type DArray = smt_z3::Z3Array<'static>;

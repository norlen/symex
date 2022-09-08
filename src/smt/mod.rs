pub mod smt_boolector;
pub mod smt_z3;

pub type DExpr = smt_z3::Z3Expr<'static>;
pub type DSolver = smt_z3::Z3SolverIncremental<'static>;
pub type DContext = smt_z3::Z3SolverContext<'static>;
pub type DArray = smt_z3::Z3Array<'static>;

// pub type DExpr = smt_boolector::BoolectorExpr;
// pub type DSolver = smt_boolector::BoolectorIncrementalSolver;
// pub type DContext = smt_boolector::BoolectorSolverContext;
// pub type DArray = smt_boolector::BoolectorArray;

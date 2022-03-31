use boolector::BVSolution;

mod bv;
mod solver;

pub use bv::BV;
pub use solver::Solver;

pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    UDiv,
    SDiv,
    URem,
    SRem,
}

pub enum UnaryOperation {}

pub enum Solutions {
    /// Could not find any solutions with the current constraints.
    None,

    /// Found these solutions, and no more.
    Exactly(Vec<BVSolution>),

    /// Non-exhaustive list of solutions, there exist more than this.
    AtLeast(Vec<BVSolution>),
}

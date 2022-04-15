use ::boolector::BVSolution;
use thiserror::Error;

mod array;
mod bv;
mod solver;

pub use array::Array;
pub use bv::BV;
pub use solver::Solver;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolverError {
    #[error("Solver state unknown")]
    Unknown,
}

pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    UDiv,
    SDiv,
    URem,
    SRem,
    And,
    Or,
    Xor,
    Sll,
    Srl,
    Sra,
}

pub enum UnaryOperation {}

#[derive(Debug)]
pub enum Solutions {
    /// Could not find any solutions with the current constraints.
    None,

    /// Found these solutions, and no more.
    Exactly(Vec<BVSolution>),

    /// Non-exhaustive list of solutions, there exist more than this.
    AtLeast(Vec<BVSolution>),
}

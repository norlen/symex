#![allow(dead_code)]
mod boolector;
mod z3;

pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    UDiv,
    SDiv,
    URem,
    SRem,
}

pub struct BVSolution;

pub enum Solutions {
    /// Could not find any solutions with the current constraints.
    None,

    /// Found these solutions, and no more.
    Exactly(Vec<BVSolution>),

    /// Non-exhaustive list of solutions, there exist more than this.
    AtLeast(Vec<BVSolution>),
}

pub trait Solver {
    type Bitvector: Bitvector + Clone;
    type Array: Array + Clone;
    type Bool: Bool + Clone;
    type Float: Float + Clone;

    fn new() -> Self;

    fn push(&self);
    fn pop(&self);

    fn is_sat(&self) -> anyhow::Result<bool>;

    fn is_sat_with_constraint(&self, constraint: &Self::Bool) -> anyhow::Result<bool>;

    fn array(&self, index_width: u32, element_width: u32, symbol: Option<&str>) -> Self::Array;

    fn bv(&self, bits: u32) -> Self::Bitvector;

    fn bv_from_bool(&self, value: bool) -> Self::Bitvector;

    fn bv_from_u64(&self, value: u64, bits: u32) -> Self::Bitvector;

    fn bv_zero(&self, bits: u32) -> Self::Bitvector;

    fn from_binary_string(&self, bits: &str) -> Self::Bitvector;

    fn assert(&self, constraint: &Self::Bool);
}

pub trait Bitvector {
    type Solver: Solver;

    fn len(&self) -> u32;

    fn zero_ext(&self, width: u32) -> Self;

    fn sign_ext(&self, width: u32) -> Self;

    fn binary_op(&self, other: Self, binop: BinaryOperation) -> Self;

    fn eq(&self, other: &Self) -> <<Self as Bitvector>::Solver as Solver>::Bool;

    fn ne(&self, other: &Self) -> <<Self as Bitvector>::Solver as Solver>::Bool;

    fn add(&self, other: &Self) -> Self;

    fn sub(&self, other: &Self) -> Self;

    fn mul(&self, other: &Self) -> Self;

    fn udiv(&self, other: &Self) -> Self;

    fn sdiv(&self, other: &Self) -> Self;

    fn urem(&self, other: &Self) -> Self;

    fn srem(&self, other: &Self) -> Self;
}

pub trait Array {}

pub trait Bool {}

pub trait Float {}

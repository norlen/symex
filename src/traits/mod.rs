use crate::vm::Result;

mod op;
mod ops;
mod size;
mod tobv;
mod value;

pub use op::*;
pub(crate) use ops::*;
pub use size::*;
pub use tobv::*;

/// Trait to get a concrete value from [llvm_ir::Operand] or [llvm_ir::Constant].
pub trait ToValue<T> {
    /// Tries to get the concrete value.
    fn to_value(&self) -> Result<T>;
}

// May change to this, the cases where I want to treat e.g. constants as values are when indexing
// into stuff like structs. This requires them to be like ints. As well for select when converting
// a constant containing a select. where the cond is a bool.
// /// Trait to treat [llvm_ir::Operand] and [llvm_ir::Constant] as values.
// pub trait AsValue {
//     fn is_int(&self) -> bool;
//     fn as_int(&self) -> u64;
//     fn try_as_int(&self) -> Result<u64>;

//     fn is_bool(&self) -> bool;
//     fn as_bool(&self) -> bool;
//     fn try_as_bool(&self) -> Result<bool>;
// }

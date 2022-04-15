use crate::vm::Result;

mod op;
mod ops;
mod size;
mod tobv;
mod value;

pub use op::*;
pub use ops::*;
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

// TODO: Should probably migrate Size functions to either the project or state.
//
// The reason why it might be the state is that for getting offsets with symbols as indices,
// the state is required (or maybe just the solver)? Then the question becomes, is the solver tied
// to the project or not? It does not seem like it, since

// /// Trait to get type, size, and offset information from types.
// pub trait Size {
//     /// Get the size in bits.
//     fn size(&self, project: &Project) -> Option<u64>;

//     /// Get the size in bytes.
//     fn size_in_bytes(&self, project: &Project) -> Result<Option<u64>>;

//     // /// Get the inner type.
//     // fn inner_ty(&self, project: &Project) -> Option<TypeRef>;
// }

use llvm_ir::TypeRef;

use crate::{
    project::Project,
    solver::BV,
    vm::{Result, State},
};

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

/// Trait to get type, size, and offset information from types.
pub trait Size {
    /// Get the size in bits.
    fn size(&self, project: &Project) -> Option<u64>;

    /// Get the size in bytes.
    fn size_in_bytes(&self, project: &Project) -> Result<Option<u64>>;

    /// Get the offset to the index in bits.
    fn offset(&self, index: u64, project: &Project) -> Option<(u64, TypeRef)>;

    /// Get the offset to the index in bytes.
    fn offset_in_bytes(&self, index: u64, project: &Project) -> Result<Option<(u64, TypeRef)>>;

    /// Get the inner type.
    fn inner_ty(&self, project: &Project) -> Option<TypeRef>;
}

/// Converts and [llvm_ir::Operand] or [llvm_ir::Constant] to a [BV].
pub trait ToBV {
    /// Converts the value to a BV. If the type is zero sized call
    /// [ToBV::try_to_bv] instead.
    fn to_bv(&self, state: &mut State<'_>) -> Result<BV>;

    /// Converts the value to a BV. Returns `None` if the type is zero sized.
    fn try_to_bv(&self, state: &mut State<'_>) -> Result<Option<BV>>;
}

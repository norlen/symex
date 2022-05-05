//! Custom modules are used to implement additional functionality not present in the IR. This
//! includes environment interaction.
//!
//! Current modules:
//! - `rust` exposes functions for panics and allocations.
//!
use crate::{
    hooks::FnInfo,
    vm::{Result, ReturnValue, VM},
};

mod rust;

pub use rust::RustModule;

type UserDefinedFunction = (
    &'static str,
    fn(&mut VM<'_>, f: FnInfo) -> Result<ReturnValue>,
);

pub trait CustomModule {
    fn get_name(&self) -> &'static str;

    fn get_all_functions(&self) -> &[UserDefinedFunction];
}

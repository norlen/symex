mod error;
mod globals;
mod instructions;
mod state;
mod vm;

pub use error::{Result, VMError};
pub use globals::*;
pub use state::*;
pub use vm::*;

use crate::BV;

enum Address {
    Concrete(u64),
    Symbolic(BV),
}

impl Address {}

impl From<u64> for Address {
    fn from(ptr: u64) -> Self {
        Self::Concrete(ptr)
    }
}

impl From<BV> for Address {
    fn from(bv: BV) -> Self {
        Self::Symbolic(bv)
    }
}

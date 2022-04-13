#![warn(rust_2018_idioms, rust_2021_compatibility)]
// For now.
#![allow(clippy::module_inception)]
//#![allow(unused_imports)]

//pub mod bv;
mod hooks;
// pub mod llvm;
pub mod memory;
pub mod project;
pub mod solver;
//mod solvert;
pub(crate) mod modpriv;
// pub mod util;
pub mod vm;

pub use solver::{Solutions, Solver, BV};

// new
pub mod traits;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

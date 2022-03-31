#![allow(unused_imports)]
#![warn(rust_2018_idioms, rust_2021_compatibility)]
// For now.
#![allow(clippy::module_inception)]

//pub mod bv;
mod hooks;
pub mod llvm;
pub mod memory;
pub mod project;
pub mod solver;
pub mod vm;

pub use solver::{Solutions, Solver, BV};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

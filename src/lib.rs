#![warn(rust_2018_idioms, rust_2021_compatibility)]
// For now.
#![allow(clippy::module_inception)]

pub mod hooks;
pub mod memory;
pub mod project;
pub mod solver;
pub mod traits;
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

pub fn assume(_b: bool) {
    // run f on current state
    // surpress false path
}

pub fn symbolic<T>(_v: &T) {
    //let size = std::mem::size_of_val(v);
    // erase all constraints related to `v`
}

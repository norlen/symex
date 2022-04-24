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

/// Assume the condition.
///
/// Adds a contraint that the passed condition must be true. If the condition can never be true,
/// this will lead to an `Unsat` error.
#[inline(never)]
#[allow(unused_variables)]
pub fn assume(condition: bool) {
    // Implemented as hook `hooks::assume`.
}

/// Creates a new symbolic value for `value`. This removes all contraints.
///
/// This creates a new symbolic variable and assigns overwrites the passed `value`. This must be
/// performed since constraints added to the solver cannot be removed, and the previous value may
/// have constraints associated with it.
#[inline(never)]
#[allow(unused_variables)]
pub fn symbolic<T>(value: &mut T) {
    // Implemented as hook `hooks::symbolic`.
}

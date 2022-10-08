//! Executor that performs the symbolic execution.
//!
//!
//!
//! ## Note
//! This module is currently being worked on to make certain pieces more generic to allow for other
//! executors, not just allowing for symbolic execution over LLVM IR.
mod cfg;
pub mod llvm;
mod stats;
mod vm;

pub use cfg::Config;
pub use stats::Stats;
pub use vm::VM;

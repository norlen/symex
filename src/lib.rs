//! SYMEX aims to be a general purpose symbolic execution environment.
//!
//! # Background
//!
//!
//!
//!
//! # LLVM IR
//!
//! It aims to be a general purpose execution engine that can analyze mainly Rust code, but should
//! handle other languages that can generate LLVM bitcode.
//!
//!
//! todo: runner example
//!
//! ```ignore
//! ```
//!
//! More information about creating a custom runner can be found in the `llvm` module.
//!
#![warn(rust_2018_idioms, rust_2021_compatibility)]
//#![warn(missing_docs)]

pub mod core;
pub mod llvm;
pub mod memory;
pub mod run;
pub mod smt;
pub mod util;

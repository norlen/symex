//! Example showcasing error output.
//!
//! ```shell
//! cargo symex --example callstack
//! ```
//!
//! Should output something along the lines of
//! ```shell
//! ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH 1 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//! Error: Abort -1
//! Stacktrace:
//!    0: callstack::bar
//!     at <path_to_examples>/examples/callstack.rs, line 35, col 9
//!    1: callstack::foo
//!     at <path_to_examples>/examples/callstack.rs, line 45, col 9
//!    2: callstack::main
//!     at <path_to_examples>/examples/callstack.rs, line 53, col 5
//!
//! Symbolic:
//!     y-3684861729: 0xfffffffffffffe04 (64-bits)
//!
//! Inputs:
//!     0: 0x0000000000000206 (64-bits)
//!
//! # More ...
//! ```
use symex_lib::{assume, symbolic, Any};

fn bar(x: i64, y: i64) -> i64 {
    //!Assume these are not too big, otherwise it catches overflow as well (at least in debug)
    assume(x < 1000 && x > 0);
    assume(y < 1000 && x > 0);

    if x + y == 10 {
        panic!();
    } else {
        x
    }
}

fn foo(x: i64) -> i64 {
    let mut y = 0;
    if x > 100 {
        symbolic(&mut y);
        bar(x, y)
    } else {
        0
    }
}

fn main() {
    let x = i64::any();
    foo(x);
}

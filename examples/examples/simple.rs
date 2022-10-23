//! Simple example.
//!
//! ```shell
//! cargo symex --example simple
//! ```
use symex_lib::{assume, symbolic, Any};

fn rust_simple_test(mut t: u32) -> u32 {
    assume(t > 0);
    if t == 0 {
        // This path should never happen.
        panic!("does not work");
    }
    symbolic(&mut t);
    if t == 0 {
        // But this should.
        0
    } else {
        11
    }
}

fn main() {
    let t = u32::any();
    rust_simple_test(t);
}

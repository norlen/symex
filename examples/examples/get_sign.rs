//! Simple example.
//!
//! ```shell
//! cargo symex --example simple --function rust_simple_test
//! ```
use symex_lib::{assume, symbolic};

#[inline(never)]
pub fn get_sign(v: i32) -> i32 {
    if v > 0 {
        return 1;
    } else if v == 0 {
        return 0;
    } else {
        return -1;
    }
}

fn main() {
    // get_sign(6);
}

//! Simple example.
//!
//! ```shell
//! cargo symex --example get_sign --function get_sign
//! ```

use symex_lib::{symbolic, Valid};

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

#[inline(never)]
pub fn get_sign_test() {
    let mut v = 0;
    symbolic(&mut v);
    match get_sign(v) {
        1 => panic!(),
        0 => panic!(),
        -1 => panic!(),
        _ => panic!(),
    }
}

fn main() {}

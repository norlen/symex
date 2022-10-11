//! Simple example.
//!
//! ```shell
//! cargo symex --example get_sign --function get_sign
//! ```

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

fn main() {}

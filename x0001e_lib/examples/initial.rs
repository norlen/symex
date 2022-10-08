//! Simple example showcasing what symbolic execution can do.
//!
//! ```shell
//! cargo x0001e --example initial --function foo
//! ```
#![allow(dead_code)]

fn foo(x: i32, y: i32) -> i32 {
    if x > 5 && x + y == 100 {
        if x * y == 1875 {
            panic!();
        } else {
            5
        }
    } else {
        x / y
    }
}

fn main() {}

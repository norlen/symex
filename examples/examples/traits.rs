//! Example which just shows that traits work properly.
//!
//! ```shell
//! cargo symex --example traits --function check
//! ```
#![allow(dead_code)]
use symex_lib::Any;

trait Random {
    fn get_random(&self) -> i32;
}

struct S;
impl Random for S {
    fn get_random(&self) -> i32 {
        42
    }
}
struct T;
impl Random for T {
    fn get_random(&self) -> i32 {
        10
    }
}

fn check() -> i32 {
    let b = bool::any();
    let a: Vec<Box<dyn Random>> = vec![Box::new(S), Box::new(T)];
    if b {
        a[0].get_random()
    } else {
        a[1].get_random()
    }
}

fn main() {}

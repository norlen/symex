#![allow(dead_code)]
use x0001e_lib::{assume, symbolic};

struct S {
    a: i16,
    b: i32,
}

fn check() -> bool {
    let mut s = S { a: 0, b: 0 };
    symbolic(&mut s);
    assume(s.b == 2);
    if s.a as i32 == s.b {
        true
    } else {
        false
    }
}

fn main() {}

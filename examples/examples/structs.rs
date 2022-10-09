//! ```shell
//! cargo symex --example structs --function foo
//! cargo symex --example structs --function check
//! ```
#![allow(dead_code)]
use symex_lib::{assume, symbolic};

#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: u8,
    c: u64,
    d: u128,
}

fn bar(s: MyStruct) -> u64 {
    if s.a == 5 {
        s.b as u64
    } else {
        s.c
    }
}

fn foo() -> u64 {
    let mut s = MyStruct {
        a: 0,
        b: 10,
        c: 15,
        d: 0,
    };
    symbolic(&mut s.a);
    bar(s)
}

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

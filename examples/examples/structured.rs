//! ```shell
//! cargo symex --example structured --function foo
//! cargo symex --example structured --function bar
//! cargo symex --example structured --function baz
//! cargo symex --example structured --function bad
//! ```
#![allow(dead_code, unreachable_patterns)]
use symex_lib::{assume, symbolic};

pub fn foo() -> u8 {
    let mut a: bool = false;
    symbolic(&mut a);
    if a {
        1
    } else {
        0
    }
}

// #[repr(C)]
struct A {
    i: u32, // 32 bits
    j: i16, // 16 bits
            // pad 16 bits
}

pub fn bar() -> u8 {
    let mut a: A = A { i: 0, j: 0 };

    symbolic(&mut a);
    assume(a.i > 30);
    assume(a.j == 1);

    if a.i > 22 {
        1
    } else {
        0
    }
}

#[derive(Eq, PartialEq)]
enum E {
    A,
    B(u8),
    C(i32),
}

pub fn baz() -> u8 {
    let mut e: E = E::A;

    symbolic(&mut e);

    e.assume_valid();

    match e {
        E::A => 1,
        E::B(_) => 2,
        E::C(_) => 3,
    }
}

pub fn bad() -> u8 {
    let mut e: E = E::A;

    symbolic(&mut e);

    match e {
        E::A => 1,
        E::B(_) => 2,
        E::C(_) => 3,
        _ => unreachable!(),
    }
}

fn main() {}

trait AssumeValid {
    fn assume_valid(&mut self) {}
}

impl AssumeValid for E {
    fn assume_valid(&mut self) {
        assume(if let E::A = self {
            true
        } else if let E::B(_) = self {
            true
        } else if let E::C(_) = self {
            true
        } else {
            false
        })
    }
}

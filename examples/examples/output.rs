//! Example showing type reconstruction and output.
//!
//! Showing array output:
//!
//! ```shell
//! cargo symex --example output --function arr
//! ```
//!
//! Showing struct output:
//!
//! ```shell
//! cargo symex --example output --function struct_simple
//! cargo symex --example output --function struct_nested
//! ```
//!
//! Note that in the output fields may be re-ordered, to force the original order `#[repr(C)]` can
//! be used.
#![allow(dead_code)]
use symex_lib::{assume, symbolic};

fn arr() {
    let mut arr = [0; 5];
    symbolic(&mut arr);
    assume(arr[0] == 0xab && arr[1] == 0xcd && arr[2] == 0xef);
}

#[derive(Default)]
//#[repr(C)]
struct A {
    a: u8,
    b: i32,
    c: i64,
    d: B,
}

#[derive(Default)]
//#[repr(C)]
struct B {
    e: i32,
    f: i64,
}

fn struct_simple() {
    let mut b = B::default();
    symbolic(&mut b);
    assume(b.e == 0xab && b.f == 0xcd);
}

fn struct_nested() {
    let mut a = A::default();
    symbolic(&mut a);

    assume(a.a == 0xde);
    assume(a.b == 0xab);
    assume(a.c == 0xabcd);

    let b = &a.d;
    assume(b.e == 0xbeef);
    assume(b.f == 0xef);
}

fn main() {}

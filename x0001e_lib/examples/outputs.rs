//! Run example with `cargo run -p cli -- --example outputs --function foobar`
#![allow(dead_code)]
use x0001e_lib::symbolic;

struct MyStruct {
    a: i32,
    b: i32,
    c: i64,
    d: u64,
}

fn foo() -> i32 {
    let mut s = MyStruct {
        a: 0,
        b: 4,
        c: 6,
        d: 8,
    };
    symbolic(&mut s);
    0
}

fn main() {
    foo();
}

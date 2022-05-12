#![allow(dead_code)]
use x0001e_lib::symbolic;

#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: u8,
    c: u64,
    d: u128,
}

fn bar(s: MyStruct) -> u64 {
    println!("{s:?}");
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

fn main() {}

#![allow(dead_code)]

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
    let s = MyStruct {
        a: 0,
        b: 10,
        c: 15,
        d: 0,
    };
    bar(s)
}

fn foobar() -> MyStruct {
    let s = MyStruct {
        a: 0,
        b: 10,
        c: 15,
        d: 0,
    };
    s
}

fn main() {}

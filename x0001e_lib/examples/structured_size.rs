#![allow(dead_code)]

struct A {
    j: i16,
    i: u32,
}

fn main() {
    let a: A = A { i: 0, j: 0 };
    println!("{}", std::mem::size_of_val(&a));
}

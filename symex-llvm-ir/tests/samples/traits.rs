#![allow(dead_code)]

trait Foo {
    fn add_five(&mut self) -> i32;
}

struct Bar {
    n: i32,
}

impl Foo for Bar {
    fn add_five(&mut self) -> i32 {
        self.n += 5;
        self.n
    }
}

fn add_five(f: &mut dyn Foo) -> i32 {
    f.add_five()
}

fn foo() -> i32 {
    let mut b = Bar { n: 5 };
    add_five(&mut b)
}

fn main() {
    let mut b = Bar { n: 5 };
    add_five(&mut b);
    let _z = foo();
}

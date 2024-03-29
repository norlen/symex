//! Showcasing generics.
//!
//! ```shell
//! cargo symex --example generics
//! ```
//!
//! Note that this won't work
//!
//! ```shell
//! cargo symex --example generics --function foo
//! ```
//!
//! Since Rust performs monomorphization the compiler will generate two functions `generics::foo`,
//! where only the mangled name is different.
#![allow(dead_code, non_snake_case)]

trait MySelf {
    fn my_self(&mut self) {}
}

struct A;
struct B {
    b: bool,
}

impl MySelf for A {}

impl MySelf for B {
    fn my_self(&mut self) {
        self.b = !self.b;
    }
}

fn foo<'a, T>(b: bool, t1: &'a mut T, t2: &'a mut T) -> &'a T
where
    T: MySelf,
{
    if b {
        t1.my_self();
        t1
    } else {
        t2
    }
}

fn foo_A(b: bool) {
    let mut t1 = A;
    let mut t2 = A;
    foo(b, &mut t1, &mut t2);
}

fn foo_B(b: bool) {
    let mut t1 = B { b: true };
    let mut t2 = B { b: false };
    foo(b, &mut t1, &mut t2);
}

fn foo_O() {
    let mut t1 = A;
    let mut t2 = A;
    let mut arr: [&mut dyn MySelf; 2] = [&mut t1, &mut t2];

    for i in &mut arr {
        i.my_self();
    }
}

fn main() {
    foo_O()
}

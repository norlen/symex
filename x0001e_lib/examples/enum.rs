#![allow(dead_code)]
use x0001e_lib::symbolic;

enum E {
    A,
    B(u8),
    C(u16),
}

fn check() -> bool {
    let mut e = E::A;
    symbolic(&mut e);
    //assume(e.is_valid());
    match e {
        E::A | E::B(_) => true,
        E::C(_) => false,
    }
}

fn main() {}

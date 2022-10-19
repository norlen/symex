//! Examples for how enums are handled.
//!
//! If `symbolic` is just called on an enum `check` shows what happens.
//!
//! ```shell
//! cargo symex --example enum --function check
//! ```
//!
//! This will trigger an `UnreachableInstruction` error, as the enum should be exhaustive and
//! `symbolic` will allow for invalid variants.
//!
//! However `check_valid` shows how to handle these cases.
//!
//! ```shell
//! cargo symex --example enum --function check_valid
//! ```
//!
//! After marking the enum as symbolic,
//! the helper function `is_valid` when derived will suppress the invalid variant and only allow
//! the valid variants.
#![allow(dead_code)]
use symex_lib::{assume, symbolic, Valid};
use valid_derive::Validate;

// trait Valid {
//     fn is_valid(&self) -> bool;
// }

// impl<T> Valid for &T {
//     fn is_valid(&self) -> bool {
//         true
//     }
// }

#[derive(Validate)]
enum E {
    A,
    B(u8),
    C(u16),
}

fn check() -> bool {
    let mut e = E::A;

    // This will mark everything as symbolic, including the variant (`A`, `B`, or `C`).
    symbolic(&mut e);

    match e {
        E::A | E::B(_) => panic!(),
        E::C(_) => panic!(),
    }
}

fn check_valid() -> bool {
    let mut e = E::A;

    // This will mark everything as symbolic, including the variant (`A`, `B`, or `C`).
    symbolic(&mut e);

    // But this will suppress the invalid variants, so for the sake of the analysis `e` can only be
    // `A`, `B`, or `C`.
    assume(e.is_valid());

    match e {
        E::A | E::B(_) => panic!(),
        E::C(_) => panic!(),
    }
}

fn check_parameter(e: E) -> bool {
    assume(e.is_valid());
    match e {
        E::A | E::B(_) => panic!(),
        E::C(_) => panic!(),
    }
}

fn main() {}

#![allow(dead_code)]

use valid_derive::*;

trait Valid {
    fn is_valid(&self) -> bool;
}

impl<T> Valid for &T {
    fn is_valid(&self) -> bool {
        true
    }
}

trait IsValid {
    fn is_valid(&self) -> bool;
}

#[derive(Validate)]
enum Enum {
    Unit,
    Unnamed(u32),
    Unnamed2(Enum2),
    Named { x: u32, y: Enum2 },
}

#[derive(Validate)]
enum Enum2 {
    U1,
    U2,
}

fn main() {}

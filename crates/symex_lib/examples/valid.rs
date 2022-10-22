#![allow(dead_code)]

use symex_lib::Validate;

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

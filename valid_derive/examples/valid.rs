#![allow(dead_code)]

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

enum Enum {
    Unit,
    Unnamed(u32),
    Unnamed2(Enum2),
    Named { x: u32, y: Enum2 },
}

enum Enum2 {
    U1,
    U2,
}

impl IsValid for Enum {
    fn is_valid(&self) -> bool {
        if let Enum::Unit = self {
            true
        } else if let Enum::Unnamed(e) = self {
            e.is_valid()
        } else if let Enum::Unnamed2(e) = self {
            e.is_valid()
        } else if let Enum::Named { x, y } = self {
            x.is_valid() && y.is_valid()
        } else {
            false
        }
    }
}

impl IsValid for Enum2 {
    fn is_valid(&self) -> bool {
        if let Enum2::U1 = self {
            true
        } else if let Enum2::U2 = self {
            true
        } else {
            false
        }
    }
}

fn main() {}

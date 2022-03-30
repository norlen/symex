use crate::BV;

pub enum Instruction {
    Ret(Ret),
}

pub struct Ret {
    return_value: BV,
}

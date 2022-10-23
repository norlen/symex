use llvm_ir::Operand;

use crate::llvm::{
    executor::{LLVMExecutor, ReturnValue},
    LLVMExecutorError,
};

pub mod custom_modules;
pub mod hooks;

type UserDefinedFunction = (
    &'static str,
    fn(&mut LLVMExecutor<'_>, &[&Operand]) -> Result<ReturnValue, LLVMExecutorError>,
);

pub trait CustomModule {
    fn get_name(&self) -> &'static str;

    fn get_all_functions(&self) -> &[UserDefinedFunction];
}

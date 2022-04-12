//! Hooks
use llvm_ir::{
    function::{FunctionAttribute, ParameterAttribute},
    Operand,
};
use log::trace;
use std::collections::HashMap;

use crate::vm::{Result, ReturnValue, VM};

use intrinsics::{is_instrinsic, Intrinsics};

mod intrinsics;

/// Hook type
pub type Hook = fn(&mut VM<'_>, f: FnInfo) -> Result<ReturnValue>;

/// Arg type
pub type Argument = (Operand, Vec<ParameterAttribute>);

pub struct FnInfo {
    pub arguments: Vec<Argument>,
    pub return_attrs: Vec<ParameterAttribute>,
    pub fn_attrs: Vec<FunctionAttribute>,
}

pub struct Hooks {
    hooks: HashMap<String, Hook>,

    intrinsics: Intrinsics,
}

impl Hooks {
    pub fn new() -> Self {
        let mut hooks = Self {
            hooks: HashMap::new(),
            intrinsics: Intrinsics::new_with_defaults(),
        };

        hooks
            .hooks
            .insert("core::panicking::panic_bounds_check".to_owned(), abort);

        hooks
    }

    pub fn get(&self, name: &str) -> Option<Hook> {
        trace!("hooks: get {}", name);
        if is_instrinsic(name) {
            self.intrinsics.get(name).map(|h| *h)
        } else {
            self.hooks.get(name).map(|h| *h)
        }
    }
}

/// Hook that tells the VM to abort.
pub fn abort(_vm: &mut VM<'_>, _info: FnInfo) -> Result<ReturnValue> {
    Ok(ReturnValue::Abort)
}

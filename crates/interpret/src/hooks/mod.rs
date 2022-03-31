use anyhow::Result;
use llvm_ir::{
    function::{FunctionAttribute, ParameterAttribute},
    Operand,
};
use log::trace;
use rustc_demangle::demangle;
use std::collections::HashMap;

use crate::vm::{ReturnValue, VM};

mod intrinsics;

pub type Hook = fn(&mut VM, f: FnInfo) -> Result<ReturnValue>;

pub type Argument = (Operand, Vec<ParameterAttribute>);

pub struct FnInfo {
    pub arguments: Vec<Argument>,
    pub return_attrs: Vec<ParameterAttribute>,
    pub fn_attrs: Vec<FunctionAttribute>,
}

pub struct Hooks {
    hooks: HashMap<String, Hook>,

    intrinsics: HashMap<String, Hook>,
}

impl Hooks {
    pub fn new() -> Self {
        let mut hooks = Self {
            hooks: HashMap::new(),
            intrinsics: HashMap::new(),
        };

        let mut add = |name: &str, f| {
            hooks.intrinsics.insert(name.to_owned(), f);
        };

        use intrinsics::*;
        add("llvm.smul.with.overflow", llvm_smul_with_overflow);
        add("llvm.expect", llvm_expect);

        hooks
            .hooks
            .insert("core::panicking::panic_bounds_check".to_owned(), abort);

        hooks
    }

    // pub fn add(&mut self, name: String, hook: Hook<'vm>) {
    //     self.hooks.insert(name, hook);
    // }

    pub fn get(&self, name: &str) -> Option<Hook> {
        trace!("hooks: get {}", name);
        if let Some(hook) = self.hooks.get(name) {
            return Some(hook.clone());
        }
        trace!("hooks: no hooks found, checking intrinsics");

        let demangled = format!("{:#}", demangle(name));
        trace!("hooks: demangled: {}", demangled.as_str());
        if let Some(hook) = self.hooks.get(demangled.as_str()) {
            return Some(hook.clone());
        }

        // If we can't find then check if it's some kind of intrinsic.
        for (n, hook) in self.intrinsics.iter() {
            trace!("hooks: cmp {} startsWith {}", name, n);
            if name.starts_with(n) {
                return Some(hook.clone());
            }
        }
        trace!("hooks: no intrinsics found");

        None
    }
}

pub fn abort(_vm: &mut VM, _info: FnInfo) -> Result<ReturnValue, anyhow::Error> {
    Ok(ReturnValue::Abort)
}

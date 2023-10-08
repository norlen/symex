use std::{ffi::CStr, path::Path};

use llvm_ir::{Function, Module};
use rustc_demangle::demangle;

use crate::vm::Result;

pub enum ProjectError {
    InvalidModule,
    InvalidName(String),
    FunctionNotFound(String),
    NoSize,
    UnexpectedZeroSize,
}

use super::{
    hooks::{Hook, Hooks},
    is_intrinsic, Intrinsic, Intrinsics,
};

/// A project handles both IR [Function]s and [Hook]s.
///
/// This enum allows a [Project] to return either of them during function lookups.
#[derive(Clone, Debug)]
pub enum FunctionType {
    Function(Function),

    Intrinsic(Intrinsic),

    /// User defined [Hook].
    Hook(Hook),
}

pub enum Overriden {
    Intrinsic(Intrinsic),
    Hook(Hook),
}

pub struct Project {
    /// All [Module]s.
    pub module: Module,

    /// Size of pointers across all module. The system does not support different pointer sizes
    /// across different modules.
    pub ptr_size: u32,

    /// Default alignment if none is specified.
    pub default_alignment: u32,

    /// User defined hooks.
    hooks: Hooks,

    /// LLVM Instrinsics.
    intrinsics: Intrinsics,
}

impl Project {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let module = Module::load(path).unwrap();
        Self::from_module(module)
    }

    pub fn from_module(module: Module) -> Result<Self> {
        // let ptr_size = modules[0].data_layout.alignments.ptr_alignment(0).size;
        let ptr_size = 64;

        let project = Project {
            module,
            ptr_size,
            default_alignment: 1,
            hooks: Hooks::new(),
            intrinsics: Intrinsics::new_with_defaults(),
        };

        Ok(project)
    }

    pub fn find_entry_function(&self, name: &str) -> Result<Function> {
        let mut return_function = None;

        for function in self.module.functions() {
            let fn_name = function.name();
            let fn_name = fn_name.to_str().unwrap();
            let demangled = demangle(fn_name);

            let demanged_no_hash = format!("{:#}", demangled);
            if fn_name == name || demangled.to_string() == name || demanged_no_hash == name {
                if return_function.is_some() {
                    panic!("Multiple functions with name {} exist", name);
                }
                return_function = Some(function);
            }
        }

        match return_function {
            Some(function) => Ok(function),
            None => panic!("Function not found"),
        }
    }

    pub fn get_instrinsic(&self, name: &str) -> Option<Intrinsic> {
        // Check for intrinsic.
        if is_intrinsic(name) {
            if let Some(intrinsic) = self.intrinsics.get(name) {
                return Some(*intrinsic);
            }
        }
        None
    }

    pub fn get_hook(&self, name: &str) -> Option<Hook> {
        // Demangle name when checking against user-defined hooks. The names in the IR should all be
        // mangled anyway.
        let demangled = demangle(name);
        let demangled_name = demangled.to_string();
        let demangled_name_no_hash = format!("{demangled:#?}");

        for name in [name, &demangled_name, &demangled_name_no_hash] {
            if let Some(hook) = self.hooks.get(name) {
                return Some(hook);
            }
        }
        None
    }

    pub fn get_function(&self, name: &CStr) -> Option<Overriden> {
        let name = name.to_string_lossy();
        // Demangle name when checking against user-defined hooks. The names in the IR should all be
        // mangled anyway.
        let demangled = demangle(&name);
        let demangled_name = demangled.to_string();
        let demangled_name_no_hash = format!("{demangled:#?}");

        // Check for intrinsic.
        if is_intrinsic(&name) {
            if let Some(intrinsic) = self.intrinsics.get(&name) {
                println!("Resolved instrinsic: {name}");
                return Some(Overriden::Intrinsic(*intrinsic));
            }
        }

        // Check for hooks.
        if let Some(hook) = self.hooks.get(&name) {
            println!("Resolved hook: {name}");
            return Some(Overriden::Hook(hook));
        }
        for name in [&demangled_name, &demangled_name_no_hash] {
            if let Some(hook) = self.hooks.get(name) {
                println!("Resolved hook: {name}");
                return Some(Overriden::Hook(hook));
            }
        }

        None
    }
}

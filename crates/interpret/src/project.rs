use std::iter;
use std::path::Path;

use crate::{
    hooks::{Hook, Hooks},
    vm::{Result, VMError},
};
use llvm_ir::{
    module::{GlobalAlias, GlobalVariable},
    types::NamedStructDef,
    Function, Module,
};
use log::info;
use rustc_demangle::demangle;

// TODO: Move hooks here.
// So we can have a unified API for looking up functions.
//
//
//

/// A project is mostly a collection of `llvm_ir::Module`s.
///
/// The `VM` takes `Project` and the entry function and exectues over that.
pub struct Project {
    /// The modules the project consists of.
    modules: Vec<Module>,

    /// Pointer size in bits used in the project.
    pub ptr_size: usize,

    hooks: Hooks,
}

pub enum FunctionType<'a> {
    Function {
        function: &'a Function,
        module: &'a Module,
    },
    Hook(Hook),
}

impl std::fmt::Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Project")
            .field("ptr_size", &self.ptr_size)
            .finish()
    }
}

impl Project {
    /// Create a new project from a LLVM BC file path.
    ///
    /// This loads a single module file.
    pub fn from_bc_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        info!("Parsing bitcode in file {}", path.as_ref().display());
        let module = Module::from_bc_path(path).map_err(|e| anyhow::anyhow!(e))?;

        let ptr_size = module.data_layout.alignments.ptr_alignment(0).size;

        let project = Self {
            ptr_size: ptr_size as usize,
            modules: vec![module],
            hooks: Hooks::new(),
        };
        Ok(project)
    }

    pub fn fn_by_name(&self, name: &str) -> Result<(&Function, &Module)> {
        // Check if this is a hook.

        // Check if we can find the literal function name.
        println!("Check literal name");
        if let Some(res) = self.find_fn_by_name(name, |name| name.to_owned())? {
            println!("found fn: {:?}", res.0);
            return Ok(res);
        }

        println!("Check demanged name");
        // Check if name is a demangled name.
        if let Some(res) = self.find_fn_by_name(name, |name| demangle(name).to_string())? {
            return Ok(res);
        }

        println!("Check demanged wihtout trailing hash name");
        // Check if name is demangled name without trailing hash.
        if let Some(res) = self.find_fn_by_name(name, |name| format!("{:#}", demangle(name)))? {
            return Ok(res);
        }

        Err(VMError::FunctionNotFound(name.to_string()))
        // Err(anyhow!("Could not find function {}", name))
    }

    fn find_fn_by_name<F>(&self, name: &str, convert: F) -> Result<Option<(&Function, &Module)>>
    where
        F: Fn(&str) -> String,
    {
        // for module in &self.modules {
        //     for f in &module.functions {
        //         println!("  mod: {}, f: {}", module.name, f.name);
        //     }
        // }
        println!("Check for function: {}", name);
        // Find functions that match the name in any modules.
        let results: Vec<_> = self
            .modules
            .iter()
            .filter_map(|module| {
                if let Some(f) = module.functions.iter().find(|f| convert(&f.name) == name) {
                    println!("------------ module: {}, fn: {}", module.name, f.name);
                    Some((f, module))
                } else {
                    None
                }
            })
            .collect();

        println!("results");
        for res in &results {
            println!("res: {}", res.0.name);
        }

        if results.len() > 1 {
            // Found more than one matching function.
            panic!(
                "Found {} function with name {} in modules",
                results.len(),
                name
            );
        }

        if let Some(d) = results.first() {
            Ok(Some(*d))
        } else {
            Ok(None)
        }
    }

    pub fn get_named_struct(&self, name: &String) -> Option<&NamedStructDef> {
        let mut ret = None;
        for module in self.modules.iter() {
            if let Some(def) = module.types.named_struct_def(name) {
                // Prefer `NamedStructDef::Defined` over `NamedStructDef::Opaque`.
                if let NamedStructDef::Defined(_) = def {
                    ret = Some(def);
                } else if ret.is_none() {
                    ret = Some(def);
                }
            }
        }
        ret
    }

    pub fn get_all_functions(&self) -> impl Iterator<Item = (&Module, &Function)> {
        self.modules
            .iter()
            .map(|module| iter::repeat(module).zip(module.functions.iter()))
            .flatten()
    }

    pub fn get_all_global_vars(&self) -> impl Iterator<Item = (&Module, &GlobalVariable)> {
        self.modules
            .iter()
            .map(|module| iter::repeat(module).zip(module.global_vars.iter()))
            .flatten()
    }

    pub fn get_all_global_aliases(&self) -> impl Iterator<Item = (&Module, &GlobalAlias)> {
        self.modules
            .iter()
            .map(|module| iter::repeat(module).zip(module.global_aliases.iter()))
            .flatten()
    }
}

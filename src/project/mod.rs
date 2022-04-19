use anyhow::anyhow;
use llvm_ir::{module::GlobalVariable, types::NamedStructDef, Function, Module, Type, TypeRef};
use log::info;
use rustc_demangle::demangle;
use std::{fs::read_dir, iter, path::Path};
use thiserror::Error;

use crate::{
    hooks::{Hook, Hooks},
    memory::to_bytes,
    traits::{
        get_bit_offset_concrete, get_bit_offset_symbol, get_byte_offset_concrete,
        get_byte_offset_symbol, size_in_bits,
    },
    vm::{Result, VMError},
    BV,
};
use map::ModulePrivateMap;

mod config;
mod map;
mod module;

pub use config::Config;

#[derive(Debug, Error)]
pub enum ProjectError {}

pub enum FunctionType<'a> {
    Function {
        function: &'a Function,
        module: &'a Module,
    },
    Hook(Hook),
}

/// A project is mostly a collection of `llvm_ir::Module`s.
///
/// The `VM` takes `Project` and the entry function and exectues over that.
pub struct Project {
    /// The modules the project consists of.
    pub(crate) modules: &'static [Module],

    /// Pointer size in bits used in the project.
    pub ptr_size: u64,

    pub default_alignment: u64,

    hooks: Hooks,

    config: Config,

    functions: ModulePrivateMap<'static>,
}

impl std::fmt::Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Project")
            .field("ptr_size", &self.ptr_size)
            .field("config", &self.config)
            .finish()
    }
}

impl Project {
    pub fn from_folder(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        info!("Checking folder for bc files {}", path.as_ref().display());

        let mut modules = Vec::new();
        for entry in read_dir(path)? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_str().unwrap();
            if name.ends_with(".bc") {
                let m = Module::from_bc_path(entry.path()).unwrap();
                modules.push(m);
            }
        }

        let ptr_size = modules[0].data_layout.alignments.ptr_alignment(0).size;

        let modules: &'static [Module] = modules.leak();

        let mut functions = ModulePrivateMap::new();
        for module in modules.iter() {
            for function in module.functions.iter() {
                functions.insert(function.name.clone(), function, module);
            }
        }

        let project = Self {
            ptr_size: ptr_size as u64,
            default_alignment: 4,
            modules,
            hooks: Hooks::new(),
            config: Config::default(),
            functions,
        };
        Ok(project)
    }

    /// Create a new project from a LLVM BC file path.
    ///
    /// This loads a single module file.
    pub fn from_bc_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        info!("Parsing bitcode in file {}", path.as_ref().display());
        let module = Module::from_bc_path(path).map_err(|e| anyhow::anyhow!(e))?;
        let ptr_size = module.data_layout.alignments.ptr_alignment(0).size;

        let modules: &'static [Module] = {
            let v = vec![module];
            v.leak()
        };

        let mut functions = ModulePrivateMap::new();
        for module in modules.iter() {
            for function in module.functions.iter() {
                functions.insert(function.name.clone(), function, module);
            }
        }

        let project = Self {
            ptr_size: ptr_size as u64,
            default_alignment: 4,
            modules,
            hooks: Hooks::new(),
            config: Config::default(),
            functions,
        };
        Ok(project)
    }

    pub fn get_function<'s>(&self, name: &str, module: &'s Module) -> Result<FunctionType<'s>> {
        let module_name = module.name.to_string();
        let demangled_names = [demangle(name).to_string(), format!("{:#}", demangle(name))];

        // Check for hook.
        if let Some(hook) = self.hooks.get(name) {
            return Ok(FunctionType::Hook(hook));
        }
        for name in demangled_names.iter() {
            if let Some(hook) = self.hooks.get(&name.as_str()) {
                return Ok(FunctionType::Hook(hook));
            }
        }

        // Check for regular function.
        if let Some((function, module)) = self.functions.get(name, &module_name) {
            return Ok(FunctionType::Function { function, module });
        }

        Err(VMError::FunctionNotFound(name.to_string()))
    }

    pub fn find_entry_function<'s>(&'s self, name: &str) -> Result<(&'s Function, &'s Module)> {
        let mut return_function = None;
        for module in self.modules {
            for function in &module.functions {
                let demangled = demangle(&function.name);

                if function.name == name
                    || demangled.to_string() == name
                    || format!("{:#}", demangled) == name
                {
                    if return_function.is_some() {
                        panic!("Multiple functions with name {} exist", name);
                    }
                    return_function = Some((function, module));
                }
            }
        }

        return_function.ok_or_else(|| VMError::FunctionNotFound(name.to_string()))
    }

    pub fn get_named_struct(&self, name: &str) -> Option<&NamedStructDef> {
        let mut ret = None;
        for module in self.modules.iter() {
            if let Some(def) = module.types.named_struct_def(name) {
                // Prefer `NamedStructDef::Defined` over `NamedStructDef::Opaque`.
                ret = Some(def)
                // if let NamedStructDef::Defined(_) = def {
                //     ret = Some(def);
                // } else if ret.is_none() {
                //     ret = Some(def);
                // }
            }
        }
        ret
    }

    pub fn get_all_functions<'s>(&'s self) -> impl Iterator<Item = (&'s Module, &'s Function)> {
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

    // pub fn get_all_global_aliases(&self) -> impl Iterator<Item = (&Module, &GlobalAlias)> {
    //     self.modules
    //         .iter()
    //         .map(|module| iter::repeat(module).zip(module.global_aliases.iter()))
    //         .flatten()
    // }

    // -------------------------------------------------------------------------
    // todo later
    // -------------------------------------------------------------------------

    // fn add_hook() {}
    // fn add_hook_to_module() {} // may be interesting to only add hooks to certain modules

    // -------------------------------------------------------------------------
    // Helpers for types and stuff
    // -------------------------------------------------------------------------

    pub fn bit_size(&self, ty: &Type) -> Result<u32> {
        let size = size_in_bits(ty, self)
            .ok_or_else(|| VMError::Other(anyhow!("Cannot take size of type")))?;
        Ok(size as u32)
    }

    pub fn byte_size(&self, ty: &Type) -> Result<u32> {
        let size = self.bit_size(ty)?;
        let size = to_bytes(size as u64)? as u32;
        Ok(size)
    }

    pub fn bit_offset_concrete(&self, ty: &Type, index: u64) -> Result<(u64, TypeRef)> {
        get_bit_offset_concrete(ty, index, self)
    }

    pub fn byte_offset_concrete(&self, ty: &Type, index: u64) -> Result<(u64, TypeRef)> {
        get_byte_offset_concrete(ty, index, self)
    }

    pub fn bit_offset_symbol(&self, ty: &Type, index: &BV) -> Result<(BV, TypeRef)> {
        get_bit_offset_symbol(ty, index, self)
    }

    pub fn byte_offset_symbol(&self, ty: &Type, index: &BV) -> Result<(BV, TypeRef)> {
        get_byte_offset_symbol(ty, index, self)
    }
}

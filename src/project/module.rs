use llvm_ir::{
    module::{GlobalVariable, Linkage},
    types::{Typed, Types},
    Function, Name, TypeRef,
};
use log::warn;
use std::{collections::HashMap, path::Path, pin::Pin};

use crate::vm::Result;

/// Collection of multiple [Module]s.
pub struct Modules {
    /// All [Module]s.
    pub modules: Pin<Box<[Module]>>,

    /// Size of pointers across all module. The system does not support different pointer sizes
    /// across different modules.
    pub ptr_size: u32,

    pub fns: HashMap<String, (&'static Module, &'static Function)>,
}

impl Modules {
    fn from_bc_path(path: impl AsRef<Path>) -> Self {
        todo!()
    }

    fn from_bc_folder(path: impl AsRef<Path>) -> Self {
        todo!()
    }

    fn from_mod(module: llvm_ir::Module) -> Self {
        let m = Module::from_module(module);

        let ms = Pin::new(Box::new([m]));

        let mut modules = Self {
            modules: ms,
            ptr_size: 64,
            fns: HashMap::new(),
        };

        for m in modules.modules.iter() {
            for (n, f) in m.private_functions.iter() {
                let name = n.clone();
                modules.fns.insert(name, (m, f));
            }
        }

        modules
    }

    // fn all_functions<'s>(&'s self) -> impl Iterator<Item = &'s Function> {
    //     let private_functions = self
    //         .modules
    //         .iter()
    //         .map(|m| m.private_functions.iter().map(|(_, f)| f))
    //         .flatten();

    //     self.global_functions.iter().chain(private_functions)
    // }

    // fn all_global_variables<'s>(&'s self) -> impl Iterator<Item = &'s GlobalVariable> {
    //     let private_global_variables = self
    //         .modules
    //         .iter()
    //         .map(|m| m.private_global_variables.iter().map(|(_, g)| g))
    //         .flatten();

    //     self.public_global_variables
    //         .iter()
    //         .chain(private_global_variables)
    // }

    fn get_function<'s>(&'s self, name: &str, module: &'s Module) -> Option<&'s Function> {
        if let Some(f) = module.private_functions.get(name) {
            Some(f)
        } else {
            // self.global_functions.get(name)
            todo!()
        }
    }
}

pub struct Module {
    /// Name of the module.
    pub name: String,

    /// Functions which are private to the current module.
    pub private_functions: HashMap<String, Function>,

    /// Global variables that are private to this module.
    pub private_global_variables: HashMap<Name, GlobalVariable>,

    /// Type information.
    pub types: Types,

    /// Size of pointers in this module.
    pub ptr_size: u32,

    /// Functions which are accessible by other modules.
    pub global_functions: Vec<Function>,

    /// Global variables that are accessible by other modules.
    pub public_global_variables: Vec<GlobalVariable>,
}

impl Module {
    fn from_module(module: llvm_ir::Module) -> Self {
        let mut private_functions = HashMap::new();
        let mut global_functions = Vec::new();

        for function in module.functions {
            match function.linkage {
                Linkage::Private | Linkage::Internal => {
                    private_functions.insert(function.name.clone(), function);
                }
                Linkage::External => {
                    global_functions.push(function);
                }
                _ => {
                    warn!(
                        "Function {} has linkage {:?}",
                        function.name.clone(),
                        function.linkage
                    );
                }
            }
        }

        let mut private_global = HashMap::new();
        let mut public_global = Vec::new();

        for global in module.global_vars {
            match global.linkage {
                Linkage::Private | Linkage::Internal => {
                    private_global.insert(global.name.clone(), global);
                }
                Linkage::External => {
                    public_global.push(global);
                }
                _ => {
                    warn!(
                        "Global variable {} has linkage {:?}",
                        global.name.clone(),
                        global.linkage
                    );
                }
            }
        }

        Self {
            name: module.name,
            private_functions,
            global_functions,
            private_global_variables: private_global,
            public_global_variables: public_global,
            types: module.types,
            ptr_size: module.data_layout.alignments.ptr_alignment(0).size,
        }
    }

    pub fn type_of<T: Typed + ?Sized>(&self, t: &T) -> TypeRef {
        self.types.type_of(t)
    }
}

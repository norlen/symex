use llvm_ir::{
    module::Linkage,
    types::{Typed, Types},
    Function, TypeRef,
};
use std::{collections::HashMap, path::Path};

pub struct Module {
    /// Name of the module.
    pub name: String,

    /// Functions which are private to the current module.
    private_functions: HashMap<String, Function>,

    /// Functions which are accessible by other modules.
    pub global_functions: Vec<Function>,

    /// Type information.
    types: Types,
}

impl Module {
    pub fn from_bc_path(path: impl AsRef<Path>) -> Self {
        let llvm_module = llvm_ir::Module::from_bc_path(path).unwrap();
        Module::from_module(llvm_module)
    }

    fn from_module(module: llvm_ir::Module) -> Self {
        let mut private_functions = HashMap::new();
        let mut global_functions = Vec::new();

        for function in module.functions {
            match function.linkage {
                Linkage::Private | Linkage::Internal => {
                    private_functions.insert(function.name.clone(), function);
                }
                _ => {
                    global_functions.push(function);
                }
            }
        }

        Self {
            name: module.name,
            private_functions,
            global_functions,
            types: module.types,
        }
    }

    pub fn type_of<T: Typed + ?Sized>(&self, t: &T) -> TypeRef {
        self.types.type_of(t)
    }
}

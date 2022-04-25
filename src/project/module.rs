#![allow(dead_code)]
use llvm_ir::{
    module::{GlobalVariable, Linkage},
    types::{NamedStructDef, Typed, Types},
    Function, Name, TypeRef,
};
use log::warn;
use std::collections::HashMap;

/// Handle that references a [Module].
pub struct ModuleHandle(usize);

/// Handle that references a [Function] in a [Module].
pub struct FunctionHandle(usize);

/// Handle that references a [GlobalVariable] in a [Module].
pub struct GlobalVariableHandle(usize);

/// Collection of multiple [Module]s.
pub struct Modules {
    /// All [Module]s.
    pub modules: Vec<Module>,

    /// Size of pointers across all module. The system does not support different pointer sizes
    /// across different modules.
    pub ptr_size: u32,

    /// Functions that are visible to other modules.
    functions: HashMap<String, (ModuleHandle, FunctionHandle)>,

    /// Global variables that are visible to other modules.
    global_variables: HashMap<Name, (ModuleHandle, GlobalVariableHandle)>,
}

impl Modules {
    /// Create a new modules struct from [llvm_ir::Module]s.
    ///
    /// This collects all the modules and processing the public functions and public global
    /// variables. It also ensures the pointer size is the same for all modules.
    ///
    /// # Panics
    ///
    /// Will panic if the passed array is empty.
    pub fn from_modules(modules: Vec<llvm_ir::Module>) -> Self {
        let modules: Vec<_> = modules.into_iter().map(Module::from_module).collect();

        let ptr_size = modules[0].ptr_size;
        let mut s = Modules {
            modules,
            ptr_size,
            functions: HashMap::new(),
            global_variables: HashMap::new(),
        };

        for (i, module) in s.modules.iter().enumerate() {
            for (j, function) in module.public_functions.iter().enumerate() {
                let entry = (ModuleHandle(i), FunctionHandle(j));
                if let Some(_) = s.functions.insert(function.name.clone(), entry) {
                    warn!(
                        "Multiple public functions with name {} exist",
                        function.name
                    );
                }
            }
        }

        for (i, module) in s.modules.iter().enumerate() {
            for (j, var) in module.public_global_variables.iter().enumerate() {
                let entry = (ModuleHandle(i), GlobalVariableHandle(j));

                if let Some(_) = s.global_variables.insert(var.name.clone(), entry) {
                    warn!(
                        "Multiple public global variables with name {} exist",
                        var.name
                    );
                }
            }
        }

        s
    }

    pub fn get_function<'s>(
        &'s self,
        name: &str,
        module: &'s Module,
    ) -> Option<(&'s Module, &Function)> {
        if let Some(f) = module.private_functions.get(name) {
            Some((module, f))
        } else if let Some((m, f)) = self.functions.get(name) {
            let module = &self.modules[m.0];
            let function = &module.public_functions[f.0];
            Some((module, function))
        } else {
            None
        }
    }

    fn get_named_struct(&self, name: &str) -> Option<&NamedStructDef> {
        // `NamedStructDef::Defined(_)` are preferred over `NamedStructDef::Opaque`. So if we
        // encounter an opaque definition, save it for later in-case we find a defined.
        let mut opaque_def = None;

        // The number of modules is expected to be small, so this should be fine.
        for module in self.modules.iter() {
            if let Some(def) = module.types.named_struct_def(name) {
                match def {
                    NamedStructDef::Opaque => opaque_def = Some(def),
                    // If we find a defined, return it straight away.
                    NamedStructDef::Defined(_) => return Some(def),
                }
            }
        }
        opaque_def
    }

    fn all_functions(&self) -> impl Iterator<Item = (&Module, &Function)> {
        let public_functions = self
            .modules
            .iter()
            .flat_map(|m| m.public_functions.iter().map(move |f| (m, f)));

        let private_functions = self
            .modules
            .iter()
            .flat_map(|m| m.private_functions.values().map(move |f| (m, f)));

        public_functions.chain(private_functions)
    }

    fn all_global_variables(&self) -> impl Iterator<Item = (&Module, &GlobalVariable)> {
        let public_globals = self
            .modules
            .iter()
            .flat_map(|m| m.public_global_variables.iter().map(move |v| (m, v)));

        let private_globals = self
            .modules
            .iter()
            .flat_map(|m| m.private_global_variables.values().map(move |v| (m, v)));

        public_globals.chain(private_globals)
    }
}

/// A Module is a collection of functions, global variables and types.
///
/// This trait allows for user defined modules, this can be used to implement parts that are not
/// available in the bitcode.
pub trait Mod {
    /// Get the name of the module.
    fn get_name(&self) -> &str;

    /// Get the pointer size that the module expects.
    fn get_ptr_size(&self) -> u32;

    // /// Get all the public functions. These are functions that should be visible to other modules.
    //fn get_public_functions(&self) -> impl Iterator<Item = &Function>;
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
    pub public_functions: Vec<Function>,

    /// Global variables that are accessible by other modules.
    pub public_global_variables: Vec<GlobalVariable>,
}

impl Module {
    fn from_module(module: llvm_ir::Module) -> Self {
        let mut private_functions = HashMap::new();
        let mut public_functions = Vec::new();
        for function in module.functions {
            match function.linkage {
                Linkage::Private | Linkage::Internal => {
                    private_functions.insert(function.name.clone(), function);
                }
                Linkage::External => {
                    public_functions.push(function);
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
            public_functions,
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

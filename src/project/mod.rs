use anyhow::anyhow;
use llvm_ir::{
    module::{GlobalVariable, Linkage},
    types::{NamedStructDef, Typed},
    Function, Module, Name, Type, TypeRef,
};
use log::warn;
use rustc_demangle::demangle;
use std::{collections::HashMap, fs, path::Path};

use crate::{
    common::{
        get_bit_offset_concrete, get_bit_offset_symbol, get_byte_offset_concrete,
        get_byte_offset_symbol, size_in_bits,
    },
    custom_modules::{CustomModule, RustModule},
    hooks::{Hook, Hooks},
    memory::to_bytes,
    VMError, BV,
};

/// Handle that references a [Module].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleHandle(usize);

/// Handle that references a [Function] in a [Module].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionHandle(usize);

/// Handle that references a [GlobalVariable] in a [Module].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalVariableHandle(usize);

/// A project handles both IR [Function]s and [Hook]s.
///
/// This enum allows a [Project] to return either of them during function lookups.
pub enum FunctionType<'p> {
    /// Regular LLVM [Function].
    Function {
        function: &'p Function,
        module: ModuleHandle,
    },

    /// User defined [Hook].
    Hook(Hook),
}

/// Collection of multiple [Module]s.
pub struct Project {
    /// All [Module]s.
    modules: Vec<Module>,

    /// Size of pointers across all module. The system does not support different pointer sizes
    /// across different modules.
    pub ptr_size: u32,

    /// Default alignment if none is specified.
    pub default_alignment: u32,

    /// Functions that are visible to other modules.
    functions: HashMap<String, (ModuleHandle, FunctionHandle)>,

    /// Functions which are private to the current module.
    private_functions: HashMap<ModuleHandle, HashMap<String, FunctionHandle>>,

    /// Global variables that are visible to other modules.
    global_variables: HashMap<Name, (ModuleHandle, GlobalVariableHandle)>,

    /// Global variables that are private to this module.
    private_global_variables: HashMap<ModuleHandle, HashMap<Name, GlobalVariableHandle>>,

    /// Public functions from user-defined modules.
    custom_module_functions: HashMap<&'static str, Hook>,

    /// User defined hooks.
    hooks: Hooks,
}

impl std::fmt::Debug for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Project")
            // .field("modules", &self.modules)
            .field("ptr_size", &self.ptr_size)
            .field("functions", &self.functions)
            .field("private_functions", &self.private_functions)
            .field("global_variables", &self.global_variables)
            .field("private_global_variables", &self.private_global_variables)
            // .field("hooks", &self.hooks)
            .finish()
    }
}

impl Project {
    /// Creates a project from a folder of `.bc` files.
    ///
    /// Sets up a new project with all LLVM bitcode files contained in the passed folder. The
    /// modules must all have the same pointer size.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use x0001e::Project;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let project = Project::from_folder("tests/doc_tests/")?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn from_folder(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let mut modules = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_str().unwrap();
            if name.ends_with(".bc") {
                let m = Module::from_bc_path(entry.path()).unwrap();
                modules.push(m);
            }
        }

        Self::from_modules(modules)
    }

    /// Creates a project from a path to a `.bc` file.
    ///
    /// Sets up a new project with all LLVM bitcode module passed in the path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use x0001e::Project;
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let project = Project::from_path("tests/doc_tests/test.bc")?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let module = Module::from_bc_path(path).unwrap();
        Self::from_modules(vec![module])
    }

    /// Create a new modules struct from [llvm_ir::Module]s.
    ///
    /// This collects all the modules and processing the public functions and public global
    /// variables. It also ensures the pointer size is the same for all modules.
    ///
    /// # Panics
    ///
    /// - Will panic if the passed array is empty.
    fn from_modules(modules: Vec<Module>) -> Result<Self, std::io::Error> {
        let ptr_size = modules[0].data_layout.alignments.ptr_alignment(0).size;
        for module in modules.iter().skip(1) {
            if module.data_layout.alignments.ptr_alignment(0).size != ptr_size {
                panic!("Inconsistent pointer size between modules");
            }
        }

        let mut functions = HashMap::new();
        let mut private_functions: HashMap<_, HashMap<_, _>> = HashMap::new();

        for (i, module) in modules.iter().enumerate() {
            let module_handle = ModuleHandle(i);
            let private_functions = private_functions.entry(module_handle).or_default();

            for (j, function) in module.functions.iter().enumerate() {
                let fn_handle = FunctionHandle(j);
                let entry = (module_handle, fn_handle);

                if let Some(privacy) = internal_get_privacy(&function.linkage) {
                    let name = function.name.clone();
                    match privacy {
                        Privacy::Internal => {
                            private_functions.insert(name, fn_handle);
                        }
                        Privacy::External => {
                            if functions.insert(name, entry).is_some() {
                                warn!(
                                    "Multiple public functions with name {} exist",
                                    function.name
                                );
                            }
                        }
                        Privacy::ExternalWeak => {
                            functions.entry(name).or_insert(entry);
                        }
                        // Not valid for functions.
                        Privacy::ExternalAppend => {
                            unreachable!();
                        }
                    }
                }
            }
        }

        let mut global_variables = HashMap::new();
        let mut private_global_variables: HashMap<_, HashMap<_, _>> = HashMap::new();

        for (i, module) in modules.iter().enumerate() {
            let module_handle = ModuleHandle(i);
            let private_globals = private_global_variables.entry(module_handle).or_default();

            for (j, var) in module.global_vars.iter().enumerate() {
                let var_handle = GlobalVariableHandle(j);
                let entry = (module_handle, var_handle);

                if let Some(privacy) = internal_get_privacy(&var.linkage) {
                    let name = var.name.clone();
                    match privacy {
                        Privacy::Internal => {
                            private_globals.insert(name, var_handle);
                        }
                        Privacy::External => {
                            if global_variables.insert(name, entry).is_some() {
                                warn!(
                                    "Multiple public global variables with name {} exist",
                                    var.name
                                );
                            }
                        }
                        Privacy::ExternalWeak => {
                            global_variables.entry(name).or_insert(entry);
                        }
                        Privacy::ExternalAppend => {
                            warn!("Append linkage is not currently supported");
                        }
                    }
                }
            }
        }

        let mut project = Project {
            modules,
            ptr_size,
            default_alignment: 1,
            functions,
            global_variables,
            private_functions,
            private_global_variables,
            custom_module_functions: HashMap::new(),
            hooks: Hooks::new(),
        };
        project.add_custom_module(RustModule {});

        Ok(project)
    }

    /// Add a [CustomModule] to the project.
    ///
    /// This adds all functions exposed in the module as public functions available in the VM.
    ///
    /// These functions have priority over functions defined in the IR, but not over hooks.
    /// Functions from other modules can override each other.
    pub fn add_custom_module(&mut self, custom_module: impl CustomModule) {
        for (name, function) in custom_module.get_all_functions() {
            let old_fn = self.custom_module_functions.insert(name, *function);

            if old_fn.is_some() {
                warn!(
                    "Overriding function {name} defined in module {}",
                    custom_module.get_name()
                );
            }
        }
    }

    /// Locate an entry point.
    ///
    /// Searches all functions in all modules for `name`. It will also check demangled names for all
    /// function names both with and without hashes.
    ///
    /// Note that for rust functions they always have the root crate as a prefix.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use x0001e::Project;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let project = Project::from_path("tests/doc_tests/test.bc")?;
    /// let (module, function) = project.find_entry_function("test::main")?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn find_entry_function(&self, name: &str) -> Result<(ModuleHandle, &Function), VMError> {
        let mut return_function = None;
        for (handle, module) in self.modules.iter().enumerate() {
            for function in &module.functions {
                let demangled = demangle(&function.name);

                if function.name == name
                    || demangled.to_string() == name
                    || format!("{:#}", demangled) == name
                {
                    if return_function.is_some() {
                        panic!("Multiple functions with name {} exist", name);
                    }
                    return_function = Some((ModuleHandle(handle), function));
                }
            }
        }

        return_function.ok_or_else(|| VMError::FunctionNotFound(name.to_string()))
    }

    /// Get a function by name.
    ///
    /// It will first check if `name` matches any user-defined hooks. Followed by module private
    /// definitions, and finally check against public functions.
    ///
    /// This can be used when creating user-defined hooks. The name is the mangled name of the
    /// function, and requires a [ModuleHandle].
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use x0001e::{Project, project::ModuleHandle};
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #   let project = Project::from_path("tests/doc_tests/test.bc")?;
    /// #   let module_handle: ModuleHandle = unsafe { std::mem::transmute(0usize) };
    /// let function = project.get_function("_ZN4test7my_func17hc738e17486d3eeeaE", module_handle)?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn get_function(
        &self,
        name: &str,
        module_handle: ModuleHandle,
    ) -> Result<FunctionType<'_>, VMError> {
        // Demangle name when checking against user-defined hooks. The names in the IR should all be
        // mangled anyway.
        let demangled = demangle(name);
        let demangled_name = demangled.to_string();
        let demangled_name_no_hash = format!("{demangled:#?}");

        // Check for hooks.
        for name in [name, &demangled_name, &demangled_name_no_hash] {
            if let Some(hook) = self.hooks.get(name) {
                return Ok(FunctionType::Hook(hook));
            }
        }

        // Check for custom module functions.
        for name in [name, &demangled_name, &demangled_name_no_hash] {
            if let Some(hook) = self.custom_module_functions.get(name) {
                return Ok(FunctionType::Hook(*hook));
            }
        }

        // Check IR functions.
        if let Some((module, function)) = self.find_function(name, module_handle) {
            return Ok(FunctionType::Function { function, module });
        }

        let f = format!("{name} ({demangled_name})");
        Err(VMError::FunctionNotFound(f))
    }

    /// Get the definition of a named struct.
    ///
    /// If the same name exists for both an Opaque and a Defined struct, the defined is returned.
    pub fn get_named_struct(&self, name: &str) -> Option<&NamedStructDef> {
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

    /// Get all [Function]s that are module private.
    pub fn get_private_functions(&self) -> impl Iterator<Item = (ModuleHandle, &Function)> {
        self.private_functions
            .iter()
            .flat_map(move |(module_handle, m)| {
                m.values().map(move |fn_handle| {
                    let function = &self.modules[module_handle.0].functions[fn_handle.0];
                    (*module_handle, function)
                })
            })
    }

    /// Get all [Function]s that are visible to all modules.
    pub fn get_public_functions(&self) -> impl Iterator<Item = (ModuleHandle, &Function)> {
        self.functions.values().map(|(module_handle, fn_handle)| {
            let function = &self.modules[module_handle.0].functions[fn_handle.0];
            (*module_handle, function)
        })
    }

    /// Get all [GlobalVariable]s that are module private.
    pub fn get_private_global_variables(
        &self,
    ) -> impl Iterator<Item = (ModuleHandle, &GlobalVariable)> {
        self.private_global_variables
            .iter()
            .flat_map(move |(module_handle, m)| {
                m.values().map(move |var_handle| {
                    let var = &self.modules[module_handle.0].global_vars[var_handle.0];
                    (*module_handle, var)
                })
            })
    }

    /// Get all [GlobalVariable]s that are visible to all modules.
    pub fn get_public_global_variables(
        &self,
    ) -> impl Iterator<Item = (ModuleHandle, &GlobalVariable)> {
        self.global_variables
            .values()
            .map(|(module_handle, var_handle)| {
                let var = &self.modules[module_handle.0].global_vars[var_handle.0];
                (*module_handle, var)
            })
    }

    /// Get the size in bits of type `ty`.
    pub fn bit_size(&self, ty: &Type) -> Result<u32, VMError> {
        let size = size_in_bits(ty, self)
            .ok_or_else(|| VMError::Other(anyhow!("Cannot take size of type")))?;
        Ok(size as u32)
    }

    /// Get the size in bytes of type `ty`.
    pub fn byte_size(&self, ty: &Type) -> Result<u32, VMError> {
        let size = self.bit_size(ty)?;
        let size = to_bytes(size as u64)? as u32;
        Ok(size)
    }

    /// Get the offset to the index in bits for type `ty`.
    pub fn bit_offset_concrete(&self, ty: &Type, index: u64) -> Result<(u64, TypeRef), VMError> {
        get_bit_offset_concrete(ty, index, self)
    }

    /// Get the offset to the index in bytes for type `ty`.
    pub fn byte_offset_concrete(&self, ty: &Type, index: u64) -> Result<(u64, TypeRef), VMError> {
        get_byte_offset_concrete(ty, index, self)
    }

    /// Get the offset to the symbolic index in bits for type `ty`.
    pub fn bit_offset_symbol(&self, ty: &Type, index: &BV) -> Result<(BV, TypeRef), VMError> {
        get_bit_offset_symbol(ty, index, self)
    }

    /// Get the offset to the symbolic index in bytes for type `ty`.
    pub fn byte_offset_symbol(&self, ty: &Type, index: &BV) -> Result<(BV, TypeRef), VMError> {
        get_byte_offset_symbol(ty, index, self)
    }

    /// Returns the type of a [Typed], e.g. [llvm_ir::Operand]s, in the given `module`.
    pub fn type_of<T: Typed>(&self, ty: &T, module: ModuleHandle) -> TypeRef {
        self.modules[module.0].type_of(ty)
    }

    /// Check `name` against public and module private functions.
    fn find_function(
        &self,
        name: &str,
        module_handle: ModuleHandle,
    ) -> Option<(ModuleHandle, &Function)> {
        if let Some(fn_handle) = self
            .private_functions
            .get(&module_handle)
            .and_then(|fns| fns.get(name))
        {
            let module = &self.modules[module_handle.0];
            let function = &module.functions[fn_handle.0];
            Some((module_handle, function))
        } else if let Some((module_handle, fn_handle)) = self.functions.get(name) {
            let module = &self.modules[module_handle.0];
            let function = &module.functions[fn_handle.0];
            Some((*module_handle, function))
        } else {
            None
        }
    }
}

enum Privacy {
    // Internal privacy means that the item is only available in the module it is defined.
    Internal,

    // External lets the item be accessed from other modules.
    External,

    // External weak let the item be accessed from other modules, but can be replaced with an
    // `External` item.
    ExternalWeak,

    // ExternalAppend is for arrays, multiple items with the same name are appended to each other.
    ExternalAppend,
}

fn internal_get_privacy(linkage: &Linkage) -> Option<Privacy> {
    // Reference: https://llvm.org/doxygen/group__LLVMCCoreTypes.html#ga0e85efb9820f572c69cf98d8c8d237de
    match linkage {
        // Private and internal should be the same for our use case. They are only accessible to
        // the current module.
        Linkage::Private | Linkage::Internal => Some(Privacy::Internal),

        // Globally visible.
        Linkage::External => Some(Privacy::External),

        // These definitions are external, but weak.
        //
        // Most should be merged. ODR version should be merged with something equivalent.
        // However, this just merges anyway and does not check that.
        //
        // Common is similar to how `Weak` works.
        Linkage::LinkOnceAny
        | Linkage::LinkOnceODR
        | Linkage::WeakAny
        | Linkage::WeakODR
        | Linkage::ExternalWeak
        | Linkage::Common => Some(Privacy::ExternalWeak),

        // For a linker this is the same as being available externally, thus we do not have to
        // add these to our maps. They should be available elsewhere.
        Linkage::AvailableExternally => None,

        // Can only be applied to array types. The two global arrays are appended together, t
        // support this the allocation would have to be the size of all of them combined.
        Linkage::Appending => Some(Privacy::ExternalAppend),

        // These should be the same as private. However, the linker should remove these.
        // Are they required then?
        Linkage::LinkerPrivate | Linkage::LinkerPrivateWeak => {
            todo!("Not supported: Got linkage: {linkage:?}")
        }

        // Obsolete types.
        Linkage::LinkOnceODRAutoHide | Linkage::DLLImport | Linkage::DLLExport | Linkage::Ghost => {
            warn!("Encountered obsolete linkage {linkage:?}");
            None
        }
    }
}

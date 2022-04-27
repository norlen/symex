//! Map of global references to their addresses.
//!
//! Keeps track of both external and internal global references ([Function]s and [GlobalVariable]s).
use llvm_ir::{module::GlobalVariable, Function, Name, Type};
use log::debug;
use std::collections::HashMap;

use crate::{
    memory::Memory,
    project::{ModuleHandle, Project},
    VMError,
};

/// A global [Function] or [GlobalVariable].
///
/// A global always have an address.
#[derive(Debug, Clone)]
pub struct GlobalReference<'p> {
    /// Address where the global is allocated.
    pub addr: u64,

    /// Reference to the [Function] or [GlobalVariable] of the global.
    pub kind: GlobalReferenceKind<'p>,
}

/// Can hold either a [Function] or [GlobalVariable].
#[derive(Debug, Clone)]
pub enum GlobalReferenceKind<'p> {
    /// Holds the referenced [GlobalVariable].
    GlobalVariable(&'p GlobalVariable),

    /// Holds the referenced [Function].
    Function(&'p Function),
}

/// GlobalReferences keeps track of all [GlobalVariable]s and [Function]s that have addresses.
///
/// Keeps a mapping between the name of a [GlobalVariable] or [Function] to its address. For
/// functions it also keep a reverse lookup table, i.e. address to function name.
#[derive(Debug, Clone)]
pub struct GlobalReferences<'p> {
    /// Maps [Function] or [GlobalVariable] name to an address.
    pub global_references: HashMap<Name, GlobalReference<'p>>,

    /// Maps module private [Function] or [GlobalVariable] name to an address.
    pub private_global_references: HashMap<ModuleHandle, HashMap<Name, GlobalReference<'p>>>,

    /// Provides lookup from an address to a function name.
    function_name_lookup: HashMap<u64, String>,

    /// Provides lookup from an address to a function name.
    private_function_name_lookup: HashMap<ModuleHandle, HashMap<u64, String>>,
}

impl<'p> GlobalReferences<'p> {
    /// Creates a global reference map from a project.
    ///
    /// Requires access to memory so it can allocate space for the global variables and create
    /// function addresses.
    pub fn from_project(project: &'p Project, memory: &mut Memory) -> Result<Self, VMError> {
        let mut s = GlobalReferences {
            global_references: HashMap::new(),
            private_global_references: HashMap::new(),
            function_name_lookup: HashMap::new(),
            private_function_name_lookup: HashMap::new(),
        };

        // Add functions.
        //
        // When functions are allocated we just allocate a pointer size, this is just so we get an
        // address. The actual bitcode instructions are never stored in symbolic memory.
        let fn_size = project.ptr_size as u64;
        let fn_align = 4;

        let mut create_fn_global_ref =
            |function: &'p Function| -> Result<GlobalReference<'p>, VMError> {
                let addr = memory.allocate(fn_size, fn_align)?;
                Ok(GlobalReference {
                    addr,
                    kind: GlobalReferenceKind::Function(function),
                })
            };

        for (module_handle, function) in project.get_private_functions() {
            let name = Name::from(&*function.name);
            let global_ref = create_fn_global_ref(function)?;
            debug!(
                "Private function {name} allocated at address: {}",
                global_ref.addr
            );

            s.add_internal_fn_addr(module_handle, global_ref.addr, function.name.clone());
            s.add_internal(module_handle, name, global_ref);
        }

        for (_, function) in project.get_public_functions() {
            let name = Name::from(&*function.name);
            let addr = memory.allocate(fn_size, fn_align)?;
            debug!("Private function {name} allocated at address: {addr}");

            s.function_name_lookup.insert(addr, function.name.clone());
            let global_ref = GlobalReference {
                addr,
                kind: GlobalReferenceKind::Function(function),
            };
            s.global_references.insert(name, global_ref);
        }

        // Add global variables.
        let mut create_var_global_ref =
            |var: &'p GlobalVariable| -> Result<GlobalReference<'p>, VMError> {
                // All GlobalVariable's should be pointers. Allocation size is based on the
                // underlying type.
                let size = match var.ty.as_ref() {
                    Type::PointerType { pointee_type, .. } => project.bit_size(pointee_type)?,
                    _ => panic!("Expected pointer type"),
                };

                // If the global is zero sized, just allocate a small amount for it.
                let size = if size == 0 { 4 } else { size };

                // If no specific alignment is specified, use the project default.
                let align = if var.alignment == 0 {
                    project.default_alignment
                } else {
                    var.alignment
                };

                let addr = memory.allocate(size as u64, align as u64)?;
                Ok(GlobalReference {
                    addr,
                    kind: GlobalReferenceKind::GlobalVariable(var),
                })
            };

        for (module_handle, var) in project.get_private_global_variables() {
            // All declaration have initializers, so skip over definitions.
            if var.initializer.is_none() {
                continue;
            }

            let name = var.name.clone();
            let global_ref = create_var_global_ref(var)?;
            debug!(
                "Private global variable {name} allocated at address: {}",
                global_ref.addr
            );
            s.add_internal(module_handle, name, global_ref);
        }

        for (_, var) in project.get_public_global_variables() {
            // All declaration have initializers, so skip over definitions.
            if var.initializer.is_none() {
                continue;
            }

            let name = var.name.clone();
            let global_ref = create_var_global_ref(var)?;
            debug!(
                "Private global variable {name} allocated at address: {}",
                global_ref.addr
            );
            s.global_references.insert(name, global_ref);
        }

        Ok(s)
    }

    /// Get a global reference by [Name].
    ///
    /// First it checks if the reference exist as a private symbol in the current module. If it does
    /// not it will check the module private definitions.
    pub fn get(&self, name: &Name, module_handle: ModuleHandle) -> Option<&GlobalReference<'p>> {
        if let Some(reference) = self
            .private_global_references
            .get(&module_handle)
            .and_then(|module_globals| module_globals.get(name))
        {
            Some(reference)
        } else {
            self.global_references.get(name)
        }
    }

    /// Get the name of a function from an address.
    ///
    /// It first checks the module private functions, followed by the global functions.
    pub fn get_function_from_address(
        &self,
        addr: u64,
        module_handle: ModuleHandle,
    ) -> Option<&str> {
        if let Some(name) = self
            .private_function_name_lookup
            .get(&module_handle)
            .and_then(|module_globals| module_globals.get(&addr))
        {
            Some(name)
        } else {
            self.function_name_lookup
                .get(&addr)
                .map(|name| name.as_str())
        }
    }

    /// Insert a mapping between a name and a module private [GlobalReference].
    fn add_internal(
        &mut self,
        module_handle: ModuleHandle,
        name: Name,
        global_ref: GlobalReference<'p>,
    ) -> Option<GlobalReference<'p>> {
        let entry = self
            .private_global_references
            .entry(module_handle)
            .or_default();

        entry.insert(name, global_ref)
    }

    /// Insert a mapping between an address and a module private function.
    fn add_internal_fn_addr(&mut self, module_handle: ModuleHandle, addr: u64, name: String) {
        let entry = self
            .private_function_name_lookup
            .entry(module_handle)
            .or_default();

        entry.insert(addr, name);
    }
}

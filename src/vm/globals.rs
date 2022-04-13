//! Global variables.
//!
//! Global variables in LLVM-IR can have different linkage types. This module
//! divides these up into two different kinds; weak and strong.
//!
//! The implementation as it is right now, treats `private`, `internal` and
//! `external` linkage as strong globals. Which overwrites weak globals.
//!
//! Weak globals have the linkage `available_externally`, `linkonce`,
//! `weak`, `common`, `extern_weak`, `linkonce_odr`, and `weak_odr`.
//!
//! `appending` is not supported at all at the moment.
//!
//! Further, `private` keeps the global private to the module it is declared in
//! while the rest allows access from the outside.
use llvm_ir::{
    module::{GlobalVariable, Linkage},
    ConstantRef, Function, Module, Name,
};
use std::{cell::Cell, collections::HashMap};

use crate::BV;

trait Global {
    fn is_private() -> bool;
    fn has_address() -> bool;
    fn is_constant() -> bool;
}

#[derive(Debug, Clone)]
pub struct Allocation<'p> {
    addr: u64,

    pub addr_bv: BV,

    pub kind: AllocationType<'p>,
}

#[derive(Debug, Clone)]
pub enum AllocationType<'p> {
    Variable(GlobalVar),
    Function(FFunction<'p>),
}

#[derive(Debug, Clone)]
pub struct GlobalVar {
    pub initializer: ConstantRef,
    pub initialized: Cell<bool>,
}

#[derive(Clone)]
pub struct FFunction<'p> {
    pub module: &'p Module,
    pub function: &'p llvm_ir::Function,
}

impl<'p> std::fmt::Debug for FFunction<'p> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FFunction").finish()
    }
}

type GlobalMap<'p> = HashMap<Name, Allocation<'p>>;

/// Globals keeps track of global variables and functions.
///
/// This structure keeps track of both global variables and functions since
/// a `[Constant::ConstantReference]` can refer to both. The globals are further
/// divided up into public and module private. The `[Linkage]` determines their
/// type. `External` linkage is treated as public, while `Internal` and `Private`
/// are treated as module private. Other linkage types are currently not supported.
///
/// Global variables are regions in memory which are allocated at compile time.
/// If the global variable is a definition it **must** have an initializer.
#[derive(Debug, Clone)]
pub struct Globals<'p> {
    pub(crate) globals: GlobalMap<'p>,

    pub(crate) private_globals: HashMap<String, GlobalMap<'p>>,

    public_addr_to_fn: HashMap<u64, Name>,
    private_addr_to_fn: HashMap<String, HashMap<u64, Name>>,
}

impl<'p> Globals<'p> {
    pub fn new() -> Self {
        Self {
            // fns: HashMap::new(),
            // private_fns: HashMap::new(),
            globals: HashMap::new(),
            private_globals: HashMap::new(),
            public_addr_to_fn: HashMap::new(),
            private_addr_to_fn: HashMap::new(),
        }
    }

    pub fn from_modules(modules: &[Module]) -> Self {
        for module in modules {
            for var in &module.global_vars {}

            for function in &module.functions {}
        }
        todo!()
    }

    pub fn get(&self, name: &Name, module: &Module) -> Option<&Allocation<'p>> {
        if let Some(allocation) = self
            .private_globals
            .get(&module.name)
            .map(|module_globals| module_globals.get(name))
            .flatten()
        {
            Some(allocation)
        } else {
            self.globals.get(name)
        }
    }

    pub fn get_func(&self, addr: u64, module: &Module) -> Option<&Allocation<'p>> {
        let name = if let Some(name) = self
            .private_addr_to_fn
            .get(&module.name)
            .and_then(|module_map| module_map.get(&addr))
        {
            Some(name)
        } else {
            self.public_addr_to_fn.get(&addr)
        }?;

        self.get(name, module)
    }

    pub fn add_global_variable(&mut self, var: &'p GlobalVariable, module: &'p Module, addr: BV) {
        let init = var.initializer.clone().unwrap();
        let g = Allocation {
            addr: 0,
            addr_bv: addr,
            kind: AllocationType::Variable(GlobalVar {
                initializer: init,
                initialized: Cell::new(false),
            }),
        };

        self.add_global(g, &var.linkage, module, var.name.clone());
    }

    pub fn add_function(&mut self, f: &'p Function, module: &'p Module, addr: BV, addr_u64: u64) {
        let g = Allocation {
            addr: 0,
            addr_bv: addr,
            kind: AllocationType::Function(FFunction {
                module,
                function: f,
            }),
        };

        self.add_global(g, &f.linkage, module, Name::from(&*f.name));
        self.add_func(&f.linkage, module, Name::from(&*f.name), addr_u64);
    }

    fn add_func(&mut self, linkage: &Linkage, module: &Module, name: Name, addr: u64) {
        use Linkage::*;
        match linkage {
            Private | Internal => {
                let addr_map = self
                    .private_addr_to_fn
                    .entry(module.name.clone())
                    .or_default();
                if addr_map.contains_key(&addr) {
                    panic!("global {} already allocated", addr);
                }
                addr_map.insert(addr, name);
            }
            External => {
                if self.public_addr_to_fn.contains_key(&addr) {
                    panic!("global {} already allocated", name);
                }
                self.public_addr_to_fn.insert(addr, name);
            }
            _ => todo!(),
        }
    }

    fn add_global(
        &mut self,
        global: Allocation<'p>,
        linkage: &Linkage,
        module: &'p Module,
        name: Name,
    ) {
        use Linkage::*;

        // Linkage types:
        //
        // - `private` and `internal` are both only accessible to by objects in their respective modules.
        // - `linkonce` and `weak` are merged with others of the same name when linked. However, `weak`
        //   should not be discarded if unreferenced.
        // - `linkonce_odr` and `weak_odr` have the one definition rule (ODR) which for our purposes are
        //   the same I think.
        //
        // - `available_externally` is only valid for definitions and can be discarded at will.
        // - `common` is similar to `weak` and have zero initializers.
        // - `extern_weak` TODO
        // - `appending` TODO
        // - `external` (default) available globally.
        //
        // For now, let's try and only keep the definitive declarations and use those, i.e. `private`,
        // `internal`, and `external`. Differentiating if they are private to the module or not.
        //
        // The others require more investigating to ensure they are handled correctly.
        match linkage {
            Private | Internal => {
                // Private to the module and strong definition.

                let global_map = self.private_globals.entry(module.name.clone()).or_default();

                if global_map.contains_key(&name) {
                    panic!("global {} already allocated", name);
                }

                global_map.insert(name, global);
            }
            External => {
                // Public and strong definition.
                if self.globals.contains_key(&name) {
                    panic!("global {} already allocated", name);
                }
                self.globals.insert(name, global);
            }
            ExternalWeak => todo!(),
            AvailableExternally => todo!(),
            LinkOnceAny => todo!(),
            LinkOnceODR => todo!(),
            LinkOnceODRAutoHide => todo!(),
            WeakAny => todo!(),
            WeakODR => todo!(),
            Common => todo!(),
            Appending => todo!(),
            DLLImport => todo!(),
            DLLExport => todo!(),
            Ghost => todo!(),
            LinkerPrivate => todo!(),
            LinkerPrivateWeak => todo!(),
        }
    }
}

// fn is_weak(linkage: Linkage) -> bool {
//     use llvm_ir::module::Linkage::*;

//     // TODO: Check all these.
//     match linkage {
//         Private | Internal | External => false,
//         ExternalWeak => true,
//         AvailableExternally => true,
//         LinkOnceAny => true,
//         LinkOnceODR => true,
//         LinkOnceODRAutoHide => true,
//         WeakAny => true,
//         WeakODR => true,
//         Common => true,
//         Appending => true,
//         DLLImport => true,
//         DLLExport => true,
//         Ghost => true,
//         LinkerPrivate => true,
//         LinkerPrivateWeak => true,
//     }
// }

// fn is_private(linkage: Linkage) -> bool {
//     matches!(linkage, Linkage::Private)
// }

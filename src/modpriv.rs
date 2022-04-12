use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use llvm_ir::module::Linkage;
use llvm_ir::{Function, Module};
use rustc_demangle::demangle;

pub trait GetVisiblity {
    fn is_private(&self) -> bool;
}

impl GetVisiblity for &Function {
    fn is_private(&self) -> bool {
        match self.linkage {
            Linkage::Private | Linkage::Internal => true,
            Linkage::External => false,
            Linkage::ExternalWeak => todo!(),
            Linkage::AvailableExternally => todo!(),
            Linkage::LinkOnceAny => todo!(),
            Linkage::LinkOnceODR => todo!(),
            Linkage::LinkOnceODRAutoHide => todo!(),
            Linkage::WeakAny => todo!(),
            Linkage::WeakODR => todo!(),
            Linkage::Common => todo!(),
            Linkage::Appending => todo!(),
            Linkage::DLLImport => todo!(),
            Linkage::DLLExport => todo!(),
            Linkage::Ghost => todo!(),
            Linkage::LinkerPrivate => todo!(),
            Linkage::LinkerPrivateWeak => todo!(),
        }
    }
}

impl GetVisiblity for Function {
    fn is_private(&self) -> bool {
        match self.linkage {
            Linkage::Private | Linkage::Internal => true,
            Linkage::External => false,
            Linkage::ExternalWeak => todo!(),
            Linkage::AvailableExternally => todo!(),
            Linkage::LinkOnceAny => todo!(),
            Linkage::LinkOnceODR => todo!(),
            Linkage::LinkOnceODRAutoHide => todo!(),
            Linkage::WeakAny => todo!(),
            Linkage::WeakODR => todo!(),
            Linkage::Common => todo!(),
            Linkage::Appending => todo!(),
            Linkage::DLLImport => todo!(),
            Linkage::DLLExport => todo!(),
            Linkage::Ghost => todo!(),
            Linkage::LinkerPrivate => todo!(),
            Linkage::LinkerPrivateWeak => todo!(),
        }
    }
}

pub struct ModulePrivateMap<'m, K, V>
where
    K: Eq + Hash,
    V: Clone + GetVisiblity,
{
    public: HashMap<K, (V, &'m Module)>,
    public_demangled: HashMap<K, (V, &'m Module)>,
    public_demangled_no_hash: HashMap<K, (V, &'m Module)>,

    private: HashMap<String, HashMap<K, (V, &'m Module)>>,
    private_demangled: HashMap<K, (V, &'m Module)>,
    private_demangled_no_hash: HashMap<K, (V, &'m Module)>,
}

impl<'m, K, V> ModulePrivateMap<'m, K, V>
where
    K: Eq + Hash,
    V: Clone + GetVisiblity,
{
    pub fn new() -> Self {
        Self {
            public: HashMap::new(),
            public_demangled: HashMap::new(),
            public_demangled_no_hash: HashMap::new(),
            private: HashMap::new(),
            private_demangled: HashMap::new(),
            private_demangled_no_hash: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V, module: &'m Module) {
        if value.is_private() {
            self.insert_private(key, value, module)
        } else {
            self.insert_public(key, value, module)
        }
    }

    pub fn insert_public(&mut self, key: K, value: V, module: &'m Module) {
        self.public.insert(key.into(), (value, module));
    }

    pub fn insert_private(&mut self, key: K, value: V, module: &'m Module) {
        let module_name = String::from(module.name.clone());
        let module_map = self.private.entry(module_name).or_default();
        module_map.insert(key, (value, module));
    }

    pub fn get_public<T>(&self, key: &T) -> Option<(V, &'m Module)>
    where
        T: ?Sized,
        K: Borrow<T>,
        T: Hash + Eq,
    {
        self.public.get(key).cloned()
    }

    pub fn get<T>(&self, key: &T, module_name: &str) -> Option<(V, &'m Module)>
    where
        T: ?Sized,
        K: Borrow<T>,
        T: Hash + Eq,
    {
        if let Some(f) = self
            .private
            .get(module_name)
            .and_then(|m| m.get(key).cloned())
        {
            Some(f)
        } else {
            self.public.get(key).cloned()
        }
    }
}

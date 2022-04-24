use llvm_ir::{Function, Module};
use std::collections::HashMap;

type Value<'m> = (&'m Function, &'m Module);

pub struct ModulePrivateMap<'m> {
    public: HashMap<String, Value<'m>>,

    private: HashMap<String, HashMap<String, Value<'m>>>,
}

impl<'m> ModulePrivateMap<'m> {
    pub fn new() -> Self {
        Self {
            public: HashMap::new(),
            private: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: &'m Function, module: &'m Module) {
        if is_private(value) {
            self.insert_private(key, value, module)
        } else {
            self.insert_public(key, value, module)
        }
    }

    pub fn insert_public(&mut self, key: String, value: &'m Function, module: &'m Module) {
        self.public.insert(key, (value, module));
    }

    pub fn insert_private(&mut self, key: String, value: &'m Function, module: &'m Module) {
        let module_name = module.name.clone();

        let module_map = self.private.entry(module_name).or_default();
        module_map.insert(key, (value, module));
    }

    pub fn get(&self, key: &str, module_name: &str) -> Option<Value<'m>> {
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

fn is_private(f: &Function) -> bool {
    match f.linkage {
        llvm_ir::module::Linkage::Private => true,
        llvm_ir::module::Linkage::Internal => true,
        llvm_ir::module::Linkage::External => false,
        llvm_ir::module::Linkage::ExternalWeak => todo!(),
        llvm_ir::module::Linkage::AvailableExternally => todo!(),
        llvm_ir::module::Linkage::LinkOnceAny => todo!(),
        llvm_ir::module::Linkage::LinkOnceODR => todo!(),
        llvm_ir::module::Linkage::LinkOnceODRAutoHide => todo!(),
        llvm_ir::module::Linkage::WeakAny => todo!(),
        llvm_ir::module::Linkage::WeakODR => todo!(),
        llvm_ir::module::Linkage::Common => todo!(),
        llvm_ir::module::Linkage::Appending => todo!(),
        llvm_ir::module::Linkage::DLLImport => todo!(),
        llvm_ir::module::Linkage::DLLExport => todo!(),
        llvm_ir::module::Linkage::Ghost => todo!(),
        llvm_ir::module::Linkage::LinkerPrivate => todo!(),
        llvm_ir::module::Linkage::LinkerPrivateWeak => todo!(),
    }
}

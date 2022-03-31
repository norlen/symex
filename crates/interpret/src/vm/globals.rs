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
#![allow(dead_code, unused_variables)]

use anyhow::Result;
use std::collections::HashMap;

use llvm_ir::{module::Linkage, Function, Module};

pub enum Allocation {}

pub struct GFunction<'project> {
    module: &'project Module,
    function: &'project Function,
    addr: u64,
}

/// globals
pub struct Globals<'project> {
    fns: HashMap<u64, GFunction<'project>>,

    private_fns: HashMap<String, HashMap<u64, GFunction<'project>>>,
}

impl<'project> Globals<'project> {
    pub fn add_function(
        &mut self,
        module: &'project Module,
        function: &'project Function,
        addr: u64,
    ) -> Result<()> {
        let fun = GFunction {
            module,
            function,
            addr,
        };

        let weak = is_weak(function.linkage);
        let private = is_private(function.linkage);

        if private {
            self.private_fns
                .entry(module.name.clone())
                .or_default()
                .insert(addr, fun);
        } else {
            self.fns.insert(addr, fun);
        }

        Ok(())
    }
}

fn is_weak(linkage: Linkage) -> bool {
    use llvm_ir::module::Linkage::*;

    // TODO: Check all these.
    match linkage {
        Private | Internal | External => false,
        ExternalWeak => true,
        AvailableExternally => true,
        LinkOnceAny => true,
        LinkOnceODR => true,
        LinkOnceODRAutoHide => true,
        WeakAny => true,
        WeakODR => true,
        Common => true,
        Appending => true,
        DLLImport => true,
        DLLExport => true,
        Ghost => true,
        LinkerPrivate => true,
        LinkerPrivateWeak => true,
    }
}

fn is_private(linkage: Linkage) -> bool {
    match linkage {
        Linkage::Private => true,
        _ => false,
    }
}

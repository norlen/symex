use llvm_ir::Name;
use std::collections::HashMap;

use crate::{vm::VMError, BV};

/// Scope for each function.
#[derive(Debug, Clone)]
struct Scope {
    /// Variables on the stack.
    vars: HashMap<Name, BV>,
}

impl Scope {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
            //versions: HashMap::new(),
        }
    }
}

/// Environment that holds all stack variables declared.
///
/// Scoped by the current function.
#[derive(Debug, Clone)]
pub struct VarMap {
    /// Variables for each scope (function).
    scopes: Vec<Scope>,
}

impl VarMap {
    pub fn new(_max_version: usize) -> Self {
        Self { scopes: Vec::new() }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, name: Name, val: BV) -> Result<(), VMError> {
        let current = self.scopes.last_mut().ok_or({
            VMError::InternalError("Tried to add variable, but no scope has been added")
        })?;

        current.vars.insert(name, val);
        Ok(())
    }

    pub fn get(&self, name: &Name) -> Option<&BV> {
        let current = self.scopes.last().unwrap();
        current.vars.get(name)
    }
}

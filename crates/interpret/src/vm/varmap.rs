use anyhow::Result;
use llvm_ir::Name;
use std::collections::HashMap;

use crate::BV;

/// Scope for each function.
#[derive(Debug, Clone)]
struct Scope {
    /// Variables on the stack.
    vars: HashMap<Name, BV>,
    // /// Version of the variables.
    //versions: HashMap<Name, usize>,
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
/// Scoped by the current function. Keeps track of the current version, for
/// detecting loop iterations.
#[derive(Debug, Clone)]
pub struct VarMap {
    /// Variables for each scope (function).
    scopes: Vec<Scope>,
    // /// Reference to the solver.
    //solver: BV::SolverRef,

    // /// Highest version number allow.
    //max_version: usize,
}

impl VarMap {
    pub fn new(_max_version: usize) -> Self {
        Self {
            scopes: Vec::new(),
            // max_version,
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, name: Name, val: BV) -> Result<()> {
        let current = self.scopes.last_mut().unwrap();

        // let version = current.versions.get(&name).cloned().unwrap_or(0);
        // if version > self.max_version {
        //     panic!("{} too many versions {}", name, version);
        // }
        // current.versions.insert(name.clone(), version + 1);

        current.vars.insert(name, val);
        Ok(())
    }

    pub fn get(&self, name: &Name) -> Option<&BV> {
        let current = self.scopes.last().unwrap();
        current.vars.get(name)
    }
}

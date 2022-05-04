//! Hooks
use llvm_ir::{
    function::{FunctionAttribute, ParameterAttribute},
    instruction::Call,
    Name, Operand, Type,
};
use log::{debug, trace};
use std::collections::HashMap;

use crate::{
    common::SolutionVariable,
    vm::{Result, ReturnValue, VMError, VM},
};

use intrinsics::{is_intrinsic, Intrinsics};

mod intrinsics;

/// Hook type
pub type Hook = fn(&mut VM<'_>, f: FnInfo) -> Result<ReturnValue>;

/// Arg type
pub type Argument = (Operand, Vec<ParameterAttribute>);

#[derive(Debug)]
pub struct FnInfo {
    pub arguments: Vec<Argument>,
    pub return_attrs: Vec<ParameterAttribute>,
    pub fn_attrs: Vec<FunctionAttribute>,
}

impl FnInfo {
    pub fn from_call(call: &Call) -> Self {
        Self {
            arguments: call.arguments.clone(),
            return_attrs: call.return_attributes.clone(),
            fn_attrs: call.function_attributes.clone(),
        }
    }
}

pub struct Hooks {
    hooks: HashMap<String, Hook>,

    intrinsics: Intrinsics,
}

impl Default for Hooks {
    fn default() -> Self {
        Self::new()
    }
}

impl Hooks {
    pub fn new() -> Self {
        let mut hooks = Self {
            hooks: HashMap::new(),
            intrinsics: Intrinsics::new_with_defaults(),
        };

        hooks.add("core::panicking::panic_bounds_check", abort);
        hooks.add("core::panicking::panic", abort);
        hooks.add("core::panicking::panic_fmt", abort);
        hooks.add("x0001e::assume", assume);
        hooks.add("x0001e::symbolic", symbolic);

        hooks.add("std::io::stdio::_print", print);

        hooks
    }

    fn add(&mut self, name: impl Into<String>, hook: Hook) {
        self.hooks.insert(name.into(), hook);
    }

    pub fn get(&self, name: &str) -> Option<Hook> {
        trace!("hooks: get {}", name);
        if is_intrinsic(name) {
            self.intrinsics.get(name).copied()
        } else {
            self.hooks.get(name).copied()
        }
    }
}

/// Hook that tells the VM to abort.
pub fn abort(_vm: &mut VM<'_>, _info: FnInfo) -> Result<ReturnValue> {
    debug!("Hook: ABORT");
    Err(VMError::Abort(-1))
}

pub fn assume(vm: &mut VM<'_>, info: FnInfo) -> Result<ReturnValue> {
    trace!("assume info: {:?}", info);

    let (condition, _) = info.arguments.get(0).unwrap();
    let condition = vm.state.get_var(condition)?;
    vm.state.solver.assert(&condition);

    Ok(ReturnValue::Void)
}

pub fn symbolic(vm: &mut VM<'_>, info: FnInfo) -> Result<ReturnValue> {
    trace!("symbolic fninfo: {:?}", info);

    let (op, _) = info.arguments.get(0).unwrap();
    let ty = vm.state.type_of(op);

    if let Type::PointerType {
        pointee_type: inner_ty,
        ..
    } = ty.as_ref()
    {
        let size = vm.project.bit_size(inner_ty.as_ref())?;

        let var_name: String = match op {
            Operand::LocalOperand { name, .. } => match name {
                Name::Name(name) => String::from(name.as_str()),
                Name::Number(_) => name.to_string(),
            },
            Operand::ConstantOperand(_) => todo!(),
            Operand::MetadataOperand => todo!(),
        };
        let new_symbol = vm.solver.bv(size, &var_name);

        // vm.state.symbols.push((var_name, new_symbol.clone()));
        let solution_var = SolutionVariable {
            name: var_name,
            value: new_symbol.clone(),
            ty: inner_ty.clone(),
        };
        vm.state.symbols.push(solution_var);

        let addr = vm.state.get_var(op)?;
        vm.state.mem.write(&addr, new_symbol)?;

        Ok(ReturnValue::Void)
    } else {
        panic!("not a pointer type");
    }
}

pub fn print(_vm: &mut VM<'_>, _info: FnInfo) -> Result<ReturnValue> {
    //println!("{info:#?}");
    Ok(ReturnValue::Void)
}

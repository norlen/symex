//! Hooks
use llvm_ir::{
    function::{FunctionAttribute, ParameterAttribute},
    instruction::Call,
    terminator::Invoke,
    Name, Operand, Type,
};
use log::trace;
use std::collections::HashMap;

use crate::{
    common::SolutionVariable,
    vm::{Result, ReturnValue, VM},
    Solutions, VMError,
};

mod intrinsics;

use intrinsics::{is_intrinsic, Intrinsics};

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

    pub fn from_invoke(invoke: &Invoke) -> Self {
        Self {
            arguments: invoke.arguments.clone(),
            return_attrs: invoke.return_attributes.clone(),
            fn_attrs: invoke.function_attributes.clone(),
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

        hooks.add("x0001e_lib::assume", assume);
        hooks.add("x0001e_lib::symbolic", symbolic);
        hooks.add("assume", assume);
        hooks.add("symbolic", symbolic_no_type);

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

pub fn assume(vm: &mut VM<'_>, info: FnInfo) -> Result<ReturnValue> {
    trace!("assume info: {:?}", info);

    let (condition, _) = info.arguments.get(0).unwrap();
    let condition = vm.state.get_var(condition)?;

    match condition.len() {
        1 => vm.state.solver.assert(&condition),
        _ => {
            let zero = vm.solver.bv_zero(condition.len());
            vm.state.solver.assert(&condition.ne(&zero))
        }
    }

    if vm.solver.is_sat()? {
        Ok(ReturnValue::Void)
    } else {
        Err(VMError::Unsat)
    }
}

pub fn symbolic_no_type(vm: &mut VM<'_>, info: FnInfo) -> Result<ReturnValue> {
    trace!("symbolic fninfo: {:?}", info);

    let addr = &info.arguments[0].0;
    let size = &info.arguments[1].0;

    let ty = vm.state.type_of(addr);
    if !matches!(ty.as_ref(), Type::PointerType { .. }) {
        panic!("Expected pointer type");
    }

    let size = vm.state.get_var(size)?;
    let size = match vm.solver.get_solutions_for_bv(&size, 1)? {
        Solutions::None => Err(VMError::Unsat),
        Solutions::Exactly(s) => Ok(s[0].as_u64().unwrap()),
        Solutions::AtLeast(_) => panic!("Found multiple solutions for size"),
    }?;

    let name = addr.to_string();
    let name = format!("{}-{}", name, rand::random::<u32>());
    let value = vm.solver.bv(size as u32, &name);

    let addr = vm.state.get_var(addr)?;
    vm.state.mem.borrow_mut().write(&addr, value.clone())?;

    let solution_var = SolutionVariable {
        name,
        value,
        ty: None,
    };
    vm.state.symbols.push(solution_var);

    Ok(ReturnValue::Void)
}

pub fn symbolic(vm: &mut VM<'_>, info: FnInfo) -> Result<ReturnValue> {
    trace!("symbolic fninfo: {:?}", info);

    let addr = &info.arguments[0].0;

    let ty = vm.state.type_of(addr);
    if let Type::PointerType {
        pointee_type: inner_ty,
        ..
    } = ty.as_ref()
    {
        let name: String = match addr {
            Operand::LocalOperand { name, .. } => match name {
                Name::Name(name) => String::from(name.as_str()),
                Name::Number(_) => name.to_string(),
            },
            Operand::ConstantOperand(_) => todo!(),
            Operand::MetadataOperand => todo!(),
        };
        let name = format!("{}-{}", name, rand::random::<u32>());
        //println!("Symbolic {name}");

        let size = vm.project.bit_size(inner_ty.as_ref())?;
        let new_symbol = vm.solver.bv(size, &name);
        let solution_var = SolutionVariable {
            name,
            value: new_symbol.clone(),
            ty: Some(inner_ty.clone()),
        };
        vm.state.symbols.push(solution_var);

        let addr = vm.state.get_var(addr)?;
        vm.state.mem.borrow_mut().write(&addr, new_symbol)?;

        Ok(ReturnValue::Void)
    } else {
        panic!("not a pointer type");
    }
}

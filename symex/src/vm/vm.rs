use llvm_ir::{Global, GlobalValue, Value};

use crate::{
    smt::{DContext, DSolver},
    util::Variable,
    vm::bit_size,
};

use super::{
    path_selection::{DFSPathSelection, Path},
    project::Project,
    state::LLVMState,
    LLVMExecutor, LLVMExecutorError, PathResult,
};

pub struct VM {
    project: &'static Project,

    pub(crate) paths: DFSPathSelection,

    pub inputs: Vec<Variable>,
}

impl VM {
    pub fn new(
        project: &'static Project,
        ctx: &'static DContext,
        fn_name: &str,
    ) -> Result<Self, LLVMExecutorError> {
        let function = project.find_entry_function(fn_name)?;
        // if let Type::Function(ty) = function.ty() {
        //     if !ty.param_types().is_empty() {
        //         panic!("Function {:?} has parameters", function.name());
        //     }
        // } else {
        //     panic!(
        //         "Function {:?} has invalid type, type: {:?}",
        //         function.name(),
        //         function.ty()
        //     );
        // };

        let mut vm = Self {
            project,
            paths: DFSPathSelection::new(),
            inputs: Vec::new(),
        };

        let solver = DSolver::new(ctx);
        let mut state = LLVMState::new(ctx, project, solver, function)?;
        vm.initialize_global_references(&mut state)?;
        vm.paths.save_path(Path::new(state, None));

        Ok(vm)
    }

    pub fn run(&mut self) -> Result<Option<(PathResult, LLVMState)>, LLVMExecutorError> {
        while let Some(path) = self.paths.get_path() {
            let mut executor = LLVMExecutor::from_state(path.state, self, self.project);
            for constraint in path.constraints {
                executor.state.constraints.assert(&constraint);
            }

            let result = executor.resume_execution()?;
            return Ok(Some((result, executor.state)));
        }
        Ok(None)
    }

    fn initialize_global_references(&self, state: &mut LLVMState) -> Result<(), LLVMExecutorError> {
        // Add functions.
        //
        // When functions are allocated we just allocate a pointer size, this is just so we get an
        // address. The actual bitcode instructions are never stored in symbolic memory.
        println!("=== Initialize globals");
        let fn_size = self.project.ptr_size as u64;
        let fn_align = 4;

        for function in self.project.module.functions() {
            let address = state.memory.allocate(fn_size, fn_align).unwrap();

            println!(
                "function {:?} allocated at address: {}",
                function.name(),
                address
            );

            let function = Value::Function(function);
            state.global_lookup.insert(function.clone(), address);
            state.global_lookup_rev.insert(address, function);
        }

        // All GlobalVariable's should be pointers. Allocation size is based on the underlying type.
        for gv in self.project.module.globals() {
            // If no specific alignment is specified, use the project default.
            let alignment = gv.alignment();
            let alignment = if alignment == 0 {
                self.project.default_alignment
            } else {
                alignment
            };

            // If the global is zero sized, just allocate a small amount for it.
            let allocated_size = if let Some(initializer) = gv.initializer() {
                match bit_size(&initializer.ty(), self.project.ptr_size)? {
                    0 => self.project.ptr_size,
                    size => size,
                }
            } else {
                self.project.ptr_size
            };

            let address = state
                .memory
                .allocate(allocated_size.into(), alignment.into())?;
            println!("gv {:?} allocated at address: {}", gv.name(), address);

            let value = Value::Global(Global::Variable(gv));
            state.global_lookup.insert(value.clone(), address);
            state.global_lookup_rev.insert(address, value);
        }

        Ok(())
    }
}

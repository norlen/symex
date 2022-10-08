use crate::{
    core::executor::VMError,
    core::smt::{Solver, SolverContext},
    executor::llvm::{project::Project, LLVMState},
    llvm::{type_to_expr_type, ReturnValue},
    path_exploration::Path,
    smt::{DContext, DSolver},
    Config, DFSPathExploration, LLVMExecutor, PathExploration, Stats, Variable,
};

#[derive(Debug)]
pub struct VM {
    project: &'static Project,

    pub(crate) paths: DFSPathExploration,

    pub cfg: Config,

    pub stats: Stats,

    pub inputs: Vec<Variable>,
}

impl VM {
    pub fn new(
        project: &'static Project,
        ctx: &'static DContext,
        fn_name: &str,
    ) -> Result<Self, VMError> {
        let (module, function) = project.find_entry_function(fn_name).unwrap();
        let solver = DSolver::new(ctx);
        let mut state = LLVMState::new(ctx, project, solver, module, function);

        // Setup initial parameters.
        let mut inputs = Vec::new();
        for param in function.parameters.iter() {
            let size = project.bit_size(&param.ty).unwrap();
            assert_ne!(size, 0);

            let s = Box::new(param.name.to_string());
            let s = Box::leak(s);

            let input = ctx.unconstrained(size as u32, s);
            inputs.push(Variable {
                name: Some(param.name.to_string()),
                value: input.clone(),
                ty: type_to_expr_type(param.ty.as_ref(), project),
            });

            state
                .stack_frames
                .last_mut()
                .unwrap()
                .registers
                .insert(param.name.clone(), input);
        }

        let mut vm = Self {
            project,
            paths: DFSPathExploration::new(),
            cfg: Config::new(),
            stats: Stats::new(),
            inputs,
        };
        let path = Path::new(state, None);
        vm.paths.save_path(path);

        Ok(vm)
    }

    pub fn run(&mut self) -> Option<(Result<ReturnValue, VMError>, LLVMState)> {
        if let Some(path) = self.paths.get_path() {
            let mut executor = LLVMExecutor::from_state(path.state, self, self.project);
            for constraint in path.constraints {
                executor.state.constraints.assert(&constraint);
            }

            let res = executor.resume_execution().map_err(|e| e.into());
            Some((res, executor.state))
        } else {
            None
        }
    }
}

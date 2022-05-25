use crate::{
    executor::{
        llvm::{project::Project, LLVMState},
        ExecutorError,
    },
    llvm::ReturnValue,
    smt::{DContext, DExpr, DSolver, SolverContext},
    LLVMExecutor, Solver,
};

/// A `Path` represents a single path of execution through a program. The path is composed by the
/// current execution state (`State`) and an optional constraint that will be asserted when this
/// path begins executing.
///
/// A single path may produce multiple other paths when encountering branching paths of execution.
#[derive(Debug, Clone)]
pub struct Path {
    /// The state to use when resuming execution.
    ///
    /// The location in the state should be where to resume execution at.
    pub state: LLVMState,

    /// Constraints to add before starting execution on this path.
    pub constraints: Vec<DExpr>,
}

impl Path {
    pub fn new(state: LLVMState, constraint: Option<DExpr>) -> Self {
        let constraints = match constraint {
            Some(c) => vec![c],
            None => vec![],
        };

        Self { state, constraints }
    }
}

// #[derive(Debug)]
// pub struct DFSPathExploration {
//     paths: Mutex<Vec<Path>>,
// }

// impl DFSPathExploration {
//     pub fn new() -> Self {
//         Self {
//             paths: Mutex::new(Vec::new()),
//         }
//     }

//     pub fn store(&mut self, path: Path) {
//         let paths = self.paths.get_mut().unwrap();
//         paths.push(path);
//     }
// }

#[derive(Debug)]
pub struct VM {
    project: &'static Project,

    // paths: DFSPathExploration,
    paths: Vec<Path>,
    solver: DSolver,
    // inputs: Vec<DExpr>,
}

// #[derive(Debug, Clone)]
// pub enum ReturnValue {
//     Value(Option<DExpr>),
//     Void,
// }

impl VM {
    pub fn new(
        project: &'static Project,
        ctx: &'static DContext,
        fn_name: &str,
    ) -> Result<Self, ExecutorError> {
        let (module, function) = project.find_entry_function(fn_name).unwrap();
        let solver = DSolver::new(ctx);
        let mut state = LLVMState::new(ctx, project, solver.clone(), module, function);

        // Setup initial parameters.
        let mut inputs = Vec::new();
        for param in function.parameters.iter() {
            let size = project.bit_size(&param.ty).unwrap();
            assert_ne!(size, 0);

            let s = Box::new(param.name.to_string());
            let s = Box::leak(s);

            let input = ctx.unconstrained(size as u32, s);
            inputs.push(input.clone());

            state
                .stack_frames
                .last_mut()
                .unwrap()
                .registers
                .insert(param.name.clone(), input);
        }

        let mut vm = Self {
            project,
            // paths: DFSPathExploration::new(),
            paths: Vec::new(),
            solver,
            // inputs,
        };
        let path = Path::new(state, None);
        vm.save_path(path);

        Ok(vm)
    }

    pub fn run(&mut self) -> Option<(Result<ReturnValue, ExecutorError>, LLVMState)> {
        if let Some(path) = self.paths.pop() {
            self.solver.pop();

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

    pub fn save_path(&mut self, path: Path) {
        self.solver.push();
        self.paths.push(path);
    }
}

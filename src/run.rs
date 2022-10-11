//! Simple runner that starts symbolic execution on LLVM bitcode.
//!
//!
use std::{
    path::Path,
    time::{Duration, Instant},
};

use tracing::info;

use rustc_demangle::demangle;

use crate::{
    core::executor::VMError,
    llvm::{LLVMState, ReturnValue},
    DContext, ErrorReason, ExpressionType, LineTrace, PathResult, PathStatus, Project, Variable,
    VM,
};

#[derive(Debug)]
pub struct RunConfig {
    /// Which paths should the solver be invoked on.
    pub solve_for: SolveFor,

    /// If concretized inputs should be shown.
    pub solve_inputs: bool,

    /// If concretized values should be displayed for variables passed to `symbolic`.
    pub solve_symbolics: bool,

    /// If concretized output values should be shown.
    pub solve_output: bool,
}

impl RunConfig {
    /// Determine if the solver should be invoked this specific result.
    ///
    /// Returns true of all paths should be solved, or if the result variant matches the given
    /// `SolveFor`.
    fn should_solve<T, E>(&self, result: &Result<T, E>) -> bool {
        use SolveFor::*;
        match self.solve_for {
            All => true,
            Error => matches!(result, Err(_)),
            Success => matches!(result, Ok(_)),
        }
    }
}

/// Determine for which types of paths the solver should be invoked on.
#[derive(Debug)]
pub enum SolveFor {
    /// All paths.
    All,

    /// Paths which return errors. Currently this is both internal executor errors and program errors.
    Error,

    /// Paths which are sucessful.
    Success,
}

pub fn run(
    path: impl AsRef<Path>,
    function: impl AsRef<str>,
    cfg: &RunConfig,
) -> Result<Vec<PathResult>, VMError> {
    // As a temporary measure both the smt context and project get leaked, this is only so I don't
    // have to care about those lifetimes, since they always live for the entire duration of the
    // run anyway.
    let context = Box::new(DContext::new());
    let context = Box::leak(context);

    let project = Box::new(Project::from_path(path).unwrap());
    let project = Box::leak(project);

    info!("create VM");
    let mut vm = VM::new(project, context, function.as_ref())?;
    info!("run paths");
    let result = run_paths(&mut vm, cfg)?;

    println!("Paths: {}, took: {:?}", result.num_paths, result.duration);
    println!(
        "Instructions processed: {}",
        vm.stats.instructions_processed
    );

    Ok(result.results)
}

struct RunnerResult {
    num_paths: usize,
    duration: Duration,
    results: Vec<PathResult>,
}

fn run_paths(vm: &mut VM, cfg: &RunConfig) -> Result<RunnerResult, VMError> {
    // Go through all paths.

    let mut results = Vec::new();
    let mut path_num = 0;

    let start = Instant::now();
    while let Some((path_result, mut state)) = vm.run() {
        path_num += 1;
        // TODO: Cache for solutions.

        if cfg.should_solve(&path_result) {
            let inputs = if cfg.solve_inputs {
                get_values(vm.inputs.iter(), &state)?
            } else {
                vec![]
            };

            let symbolics = if cfg.solve_symbolics {
                get_values(state.marked_symbolic.iter(), &state)?
            } else {
                vec![]
            };

            let result = match path_result {
                Ok(value) => {
                    let value = match value {
                        ReturnValue::Value(expr) => Some(Variable {
                            name: Some("output".to_string()),
                            value: if cfg.solve_output {
                                state.constraints.get_value(&expr)?
                            } else {
                                expr
                            },
                            ty: ExpressionType::Unknown,
                        }),
                        ReturnValue::Void => None,
                    };
                    PathStatus::Ok(value)
                }
                Err(e) => PathStatus::Failed(create_error_reason(&mut state, e)),
            };

            let path_result = PathResult {
                path: path_num,
                result,
                inputs,
                symbolics,
            };
            println!("{}", path_result);

            results.push(path_result);
        }
    }

    Ok(RunnerResult {
        num_paths: path_num,
        duration: start.elapsed(),
        results,
    })
}

fn create_error_reason(state: &mut LLVMState, error: VMError) -> ErrorReason {
    let error_message = format!("{}", error);
    // let error_location = state.stack_frames.last()
    //     .current_loc
    //     .source_loc
    //     .map(|location| format!("{}", location));

    // REMOVE ME
    // inputs = generate_solutions(vm.parameters.iter(), &mut cache, project)?;
    // symbolics = generate_solutions(vm.state.symbols.iter(), &mut cache, project)?;

    let mut stack_trace = Vec::new();
    for callstack in state.stack_frames.iter().rev() {
        // Demangled function names, leave out the hash as well.
        let demangled = demangle(&callstack.location.func.name);
        let function_name = format!("{demangled:#}");

        let line = LineTrace {
            function_name,
            line: callstack
                .location
                .source_loc
                .map(|location| format!("{location}")),
        };
        stack_trace.push(line);
    }

    ErrorReason {
        error_message,
        error_location: None,
        stack_trace,
    }
}

fn get_values<'a, I>(vars: I, state: &LLVMState) -> Result<Vec<Variable>, VMError>
where
    I: Iterator<Item = &'a Variable>,
{
    let mut results = Vec::new();
    for var in vars {
        let constant = state.constraints.get_value(&var.value)?;
        let var = Variable {
            name: var.name.clone(),
            value: constant,
            ty: var.ty.clone(),
        };
        results.push(var);
    }

    Ok(results)
}

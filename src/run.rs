use std::{path::Path, time::Instant};

use crate::{
    llvm::LLVMState, DContext, ErrorReason, ExecutorError, PathResult, PathStatus, Project,
    Variable, VM,
};

fn get_values<'a, I>(vars: I, state: &LLVMState) -> Result<Vec<Variable>, ExecutorError>
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

pub struct RunConfig {
    pub solve_inputs: bool,

    pub solve_symbolics: bool,

    pub solve_output: bool,

    pub solve_for: SolveFor,
}

pub enum SolveFor {
    All,
    Error,
    Success,
}

pub fn run<P, S>(path: P, function: S, cfg: &RunConfig) -> Result<Vec<PathResult>, ExecutorError>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let context = Box::new(DContext::new());
    let context = Box::leak(context);

    let project = Box::new(Project::from_path(path).unwrap());
    let project = Box::leak(project);

    let mut vm = VM::new(project, context, function.as_ref())?;

    let start = Instant::now();

    // Go through all paths.

    let mut results = Vec::new();
    let mut path_num = 0;
    while let Some((path_result, state)) = vm.run() {
        path_num += 1;
        // TODO: Cache for solutions.

        let should_solve = match cfg.solve_for {
            SolveFor::All => true,
            SolveFor::Error => matches!(path_result, Err(_)),
            SolveFor::Success => matches!(path_result, Ok(_)),
        };

        let inputs = if should_solve && cfg.solve_inputs {
            get_values(vm.inputs.iter(), &state)?
        } else {
            vec![]
        };

        let symbolics = if should_solve && cfg.solve_symbolics {
            get_values(state.marked_symbolic.iter(), &state)?
        } else {
            vec![]
        };

        let result = match path_result {
            Ok(v) => PathStatus::Ok(match v {
                crate::llvm::ReturnValue::Value(v) => Some(Variable {
                    name: Some("output".to_string()),
                    value: if should_solve && cfg.solve_output {
                        state.constraints.get_value(&v)?
                    } else {
                        v
                    },
                    ty: crate::ExpressionType::Unknown,
                }),
                crate::llvm::ReturnValue::Void => None,
            }),
            Err(e) => PathStatus::Failed(ErrorReason {
                error_message: format!("{e}"),
                error_location: None,
                stack_trace: vec![],
            }),
        };

        let result = PathResult {
            path: path_num,
            result,
            inputs,
            symbolics,
        };
        results.push(result);

        // let result = match path_result {
        //     Ok(return_value) => {
        //         let return_value = match return_value {
        //             ReturnValue::Value(return_value) => {
        //                 // Try to reconstruct the type based on the last executed instruction.
        //                 let terminator = &vm.state.current_loc.block.term;
        //                 let ty = match terminator {
        //                     Terminator::Ret(instr) => match &instr.return_operand {
        //                         Some(op) => Some(vm.state.type_of(op)),
        //                         None => None,
        //                     },
        //                     _ => None,
        //                 };

        //                 if let Some(ty) = ty {
        //                     // Solve the return value.
        //                     let solutions = cache.get_solutions(&return_value);
        //                     println!("Solutions: {:?}", solutions);
        //                     let value = cache.get_solution(&return_value)?;
        //                     let value = ConcreteValue::from_binary_str(
        //                         value.as_01x_str(),
        //                         ty.as_ref(),
        //                         project,
        //                     );
        //                     let variable = Variable { name: None, value };
        //                     Some(variable)
        //                 } else {
        //                     None
        //                 }
        //             }
        //             ReturnValue::Void => None,
        //         };

        //         PathStatus::Ok(return_value)
        //     }
        //     Err(error) => {
        //         let error_message = format!("{}", error);
        //         let error_location = vm
        //             .state
        //             .current_loc
        //             .source_loc
        //             .map(|location| format!("{}", location));

        //         // REMOVE ME
        //         // inputs = generate_solutions(vm.parameters.iter(), &mut cache, project)?;
        //         // symbolics = generate_solutions(vm.state.symbols.iter(), &mut cache, project)?;

        //         let mut stack_trace = Vec::new();
        //         for callstack in vm.state.callstack.iter().rev() {
        //             // Demangled function names, leave out the hash as well.
        //             let demangled = demangle(&callstack.location.func.name);
        //             let function_name = format!("{demangled:#}");

        //             let line = LineTrace {
        //                 function_name,
        //                 line: callstack
        //                     .location
        //                     .source_loc
        //                     .map(|location| format!("{location}")),
        //             };
        //             stack_trace.push(line);
        //         }

        //         let error_reason = ErrorReason {
        //             error_message,
        //             error_location,
        //             stack_trace,
        //         };
        //         PathStatus::Failed(error_reason)
        //     }
        // };

        // let path_result = PathResult {
        //     path: path_num,
        //     result,
        //     inputs,
        //     symbolics,
        // };

        // println!("{}", path_result);
    }
    let duration = start.elapsed();
    println!("Paths: {path_num}, took: {duration:?}");
    println!(
        "Instructions processed: {}",
        vm.stats.instructions_processed
    );

    Ok(results)
}

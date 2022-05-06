use anyhow::Result;
use rustc_demangle::demangle;
use std::path::Path;

use crate::*;
use x0001e::{
    common::SolutionVariable, ir::*, solver::SolutionGenerator, Project, ReturnValue, VM,
};

/// Helper to generate solutions from a list of `SolutionVariable`s.
fn generate_solutions<'a>(
    symbols: impl Iterator<Item = &'a SolutionVariable>,
    cache: &mut SolutionGenerator,
    project: &Project,
) -> Result<Vec<Variable>> {
    let mut variables = Vec::new();

    for symbol in symbols {
        let name = Some(symbol.name.clone());
        let value = cache.get_solution(&symbol.value)?;
        let value = ConcreteValue::from_binary_str(value.as_01x_str(), symbol.ty.as_ref(), project);

        let variable = Variable { name, value };
        variables.push(variable);
    }

    Ok(variables)
}

/// Start running the analysis from a path to a BC file and the function to analyze.
pub fn run(path: impl AsRef<Path>, function: &str) -> Result<()> {
    let project = Project::from_path(path)?;
    run_project(&project, function)
}

/// Start running analysis from with a given Project.
pub fn run_project(project: &Project, function: &str) -> Result<()> {
    let mut vm = VM::new(function, project)?;

    let mut path_num = 0;
    // Go through all paths.
    while let Some(path_result) = vm.run() {
        path_num += 1;

        // Cache for solutions.
        //
        // Solutions cannot be cached between paths, so instantiate a new one for each path.
        let mut cache = SolutionGenerator::new(vm.solver.clone());

        let inputs = generate_solutions(vm.parameters.iter(), &mut cache, project)?;
        let symbolics = generate_solutions(vm.state.symbols.iter(), &mut cache, project)?;

        let result = match path_result {
            Ok(return_value) => {
                let return_value = match return_value {
                    ReturnValue::Value(return_value) => {
                        // Try to reconstruct the type based on the last executed instruction.
                        let terminator = &vm.state.current_loc.block.term;
                        let ty = match terminator {
                            Terminator::Ret(instr) => match &instr.return_operand {
                                Some(op) => Some(vm.state.type_of(op)),
                                None => None,
                            },
                            _ => None,
                        };

                        if let Some(ty) = ty {
                            // Solve the return value.
                            let value = cache.get_solution(&return_value)?;
                            let value = ConcreteValue::from_binary_str(
                                value.as_01x_str(),
                                ty.as_ref(),
                                project,
                            );
                            let variable = Variable { name: None, value };
                            Some(variable)
                        } else {
                            None
                        }
                    }
                    ReturnValue::Void => None,
                };

                PathStatus::Ok(return_value)
            }
            Err(error) => {
                let error_message = format!("{}", error);
                let error_location = vm
                    .state
                    .current_loc
                    .source_loc
                    .map(|location| format!("{}", location));

                let mut stack_trace = Vec::new();
                for callstack in vm.state.callstack.iter().rev() {
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

                let error_reason = ErrorReason {
                    error_message,
                    error_location,
                    stack_trace,
                };
                PathStatus::Failed(error_reason)
            }
        };

        let path_result = PathResult {
            path: path_num,
            result,
            inputs,
            symbolics,
        };

        println!("{}", path_result);
    }

    Ok(())
}

use anyhow::Result;
use std::path::Path;
use std::time::Instant;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use x0001e::{DContext, Project, VM};

/// Helper to generate solutions from a list of `SolutionVariable`s.
// fn generate_solutions<'a>(
//     symbols: impl Iterator<Item = &'a SolutionVariable>,
//     cache: &mut SolutionGenerator,
//     project: &Project,
// ) -> Result<Vec<Variable>> {
//     let mut variables = Vec::new();

//     for symbol in symbols {
//         let name = Some(symbol.name.clone());
//         let value = cache.get_solution(&symbol.value)?;

//         let value = match &symbol.ty {
//             Some(ty) => ConcreteValue::from_binary_str(value.as_01x_str(), ty.as_ref(), project),
//             None => ConcreteValue::Unknown(value.as_01x_str().to_owned()),
//         };

//         let variable = Variable { name, value };
//         variables.push(variable);
//     }

//     Ok(variables)
// }

const SHOW_OUTPUT: bool = false;

/// Start running the analysis from a path to a BC file and the function to analyze.
pub fn run(path: impl AsRef<Path>, function: &str) -> Result<()> {
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::TRACE)
    //     .finish();

    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let context = Box::new(DContext::new());
    let context = Box::leak(context);

    let project = Box::new(Project::from_path(path).unwrap());
    let project = Box::leak(project);

    // let ctx = x0001e::create_ctx();
    let mut vm = VM::new(project, context, function)?;

    let start = Instant::now();

    let mut path_num = 0;
    // Go through all paths.
    while let Some((path_result, _state)) = vm.run() {
        path_num += 1;
        println!("Result: {path_result:?}");
        // path_result.unwrap();

        if !SHOW_OUTPUT {
            continue;
        }

        // Cache for solutions.
        //
        // Solutions cannot be cached between paths, so instantiate a new one for each path.
        // let mut cache = SolutionGenerator::new(vm.solver.clone())?;

        // let inputs = generate_solutions(vm.parameters.iter(), &mut cache, project)?;
        // let symbolics = generate_solutions(vm.state.symbols.iter(), &mut cache, project)?;
        // let mut inputs = vec![];
        // let mut symbolics = vec![];

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

    Ok(())
}

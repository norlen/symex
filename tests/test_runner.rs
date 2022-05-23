use std::path::Path;

use llvm_ir::{types::NamedStructDef, Type};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use x0001e::vm::ReturnValue;
use x0001e::ConcreteValue;
use x0001e::ExecutorError;
use x0001e::Project;

// /// Helper to generate solutions from a list of `SolutionVariable`s.
// fn generate_solutions<'a>(
//     symbols: impl Iterator<Item = &'a SolutionVariable>,
//     cache: &mut SolutionGenerator,
//     project: &Project,
// ) -> Result<Vec<ConcreteValue>> {
//     let mut values = Vec::new();

//     for symbol in symbols {
//         let value = cache.get_solution(&symbol.value)?;
//         let value = match &symbol.ty {
//             Some(ty) => ConcreteValue::from_binary_str(value.as_01x_str(), ty.as_ref(), project),
//             None => ConcreteValue::Unknown(value.as_01x_str().to_owned()),
//         };
//         values.push(value);
//     }

//     Ok(values)
// }

#[derive(Debug)]
pub struct PathResult {
    pub result: Result<Option<ConcreteValue>, ExecutorError>,

    pub inputs: Vec<ConcreteValue>,

    pub symbolics: Vec<ConcreteValue>,
}

pub fn run(path: impl AsRef<Path>, function: &str) -> Result<Vec<PathResult>, ExecutorError> {
    // let _ = env_logger::builder().is_test(true).try_init();
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber); //.expect("setting default subscriber failed");

    let results = x0001e::run(path, function)?;
    let results = results
        .into_iter()
        .map(|res| {
            let r = match res {
                ReturnValue::Value(v) => Some(v.unwrap()),
                ReturnValue::Void => None,
            };
            PathResult {
                result: Ok(r),
                inputs: vec![],
                symbolics: vec![],
            }
        })
        .collect();
    Ok(results)
}

// pub fn run(path: impl AsRef<Path>, function: &str) -> Result<Vec<PathResult>> {
//     let _ = env_logger::builder().is_test(true).try_init();

//     let project = Project::from_path(path)?;
//     let mut vm = VM::new(function, &project)?;
//     let mut results = Vec::new();

//     // Go through all paths.
//     while let Some(path_result) = vm.run() {
//         // Cache for solutions.
//         //
//         // Solutions cannot be cached between paths, so instantiate a new one for each path.
//         let mut cache = SolutionGenerator::new(vm.solver.clone())?;

//         let inputs = generate_solutions(vm.parameters.iter(), &mut cache, &project)?;
//         let symbolics = generate_solutions(vm.state.symbols.iter(), &mut cache, &project)?;

//         let result = match path_result {
//             Ok(return_value) => {
//                 let return_value = match return_value {
//                     ReturnValue::Value(return_value) => {
//                         // Try to reconstruct the type based on the last executed instruction.
//                         let terminator = &vm.state.current_loc.block.term;
//                         let ty = match terminator {
//                             Terminator::Ret(instr) => match &instr.return_operand {
//                                 Some(op) => Some(vm.state.type_of(op)),
//                                 None => None,
//                             },
//                             _ => None,
//                         };

//                         if let Some(ty) = ty {
//                             // Solve the return value.
//                             let value = cache.get_solution(&return_value)?;
//                             let value = ConcreteValue::from_binary_str(
//                                 value.as_01x_str(),
//                                 ty.as_ref(),
//                                 &project,
//                             );
//                             Some(value)
//                         } else {
//                             None
//                         }
//                     }
//                     ReturnValue::Void => None,
//                 };

//                 Ok(return_value)
//             }
//             Err(error) => Err(error),
//         };

//         let path_result = PathResult {
//             result,
//             inputs,
//             symbolics,
//         };
//         results.push(path_result);
//     }

//     Ok(results)
// }

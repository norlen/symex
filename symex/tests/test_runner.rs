use std::path::Path;

use symex::{
    smt::{DContext, DExpr},
    vm::{LLVMState, PathResult, Project, VM},
};

fn get_u128_value(expr: DExpr, state: &LLVMState) -> u128 {
    let value = state
        .constraints
        .get_value(&expr)
        .expect("Failed to get concrete value");

    let binary_str = value.to_binary_string();
    u128::from_str_radix(&binary_str, 2).unwrap()
}

pub fn run(path: impl AsRef<Path>, function: &str) -> Vec<Option<u128>> {
    // let subscriber = tracing_subscriber::FmtSubscriber::builder()
    //     .with_max_level(tracing::Level::TRACE)
    //     .finish();

    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let context = Box::new(DContext::new());
    let context = Box::leak(context);

    let project = Box::new(Project::from_path(path).unwrap());
    let project = Box::leak(project);

    let mut vm = VM::new(project, context, function).expect("Failed to create VM");

    let mut results = Vec::new();
    while let Some((path_result, state)) = vm.run().expect("Failed to run paths") {
        // let inputs = generate_solutions(vm.parameters.iter(), &mut cache, &project)?;
        // let symbolics = generate_solutions(vm.state.symbols.iter(), &mut cache, &project)?;

        let result = match path_result {
            PathResult::Success(value) => value.map(|value| get_u128_value(value, &state)),
            PathResult::Failure(_) => panic!("analysis failed"),
            PathResult::Suppress => panic!("path suppressed"),
            PathResult::AssumptionUnsat => panic!("assumption unsat"),
        };

        results.push(result);
    }

    results
}

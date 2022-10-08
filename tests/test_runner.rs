use std::path::Path;
// use tracing::Level;
// use tracing_subscriber::FmtSubscriber;

use x0001e::{
    core::{executor::VMError, smt::Expression},
    llvm::{LLVMState, ReturnValue},
    DContext, DExpr, Project, VM,
};

fn get_u128_value(expr: DExpr, state: &LLVMState) -> u128 {
    let value = state
        .constraints
        .get_value(&expr)
        .expect("Failed to get concrete value");

    let binary_str = value.to_binary_string();
    u128::from_str_radix(&binary_str, 2).unwrap()
}

pub fn run(path: impl AsRef<Path>, function: &str) -> Vec<Result<Option<u128>, VMError>> {
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::TRACE)
    //     .finish();

    // tracing::subscriber::set_global_default(subscriber); //.expect("setting default subscriber failed");

    let context = Box::new(DContext::new());
    let context = Box::leak(context);

    let project = Box::new(Project::from_path(path).unwrap());
    let project = Box::leak(project);

    let mut vm = VM::new(project, context, function).expect("Failed to create VM");

    let mut results = Vec::new();
    while let Some((path_result, state)) = vm.run() {
        // let inputs = generate_solutions(vm.parameters.iter(), &mut cache, &project)?;
        // let symbolics = generate_solutions(vm.state.symbols.iter(), &mut cache, &project)?;

        let result = path_result.map(|value| match value {
            ReturnValue::Value(value) => Some(get_u128_value(value, &state)),
            ReturnValue::Void => None,
        });

        results.push(result);
    }

    results
}

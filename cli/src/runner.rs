use anyhow::{anyhow, Result};
use std::{collections::HashMap, path::Path};
use x0001e::{
    project::Project,
    vm::{self, ReturnValue, VM},
    Solutions, Solver, BV,
};

#[derive(Debug)]
#[allow(dead_code)]
struct Solution {
    ret: vm::Result<ReturnValue>,

    /// One solution for each of the input parameters. If any of them are Unsat the entire vector
    /// will be empty.
    inputs: Vec<u64>,

    /// Values explictly marked as symbolic. One solution for each of them.
    symbolic: HashMap<String, u64>,

    /// Output value. Will be set if the output is a u64, otherwise `None` for e.g. returning void.
    output: Option<u64>,
}

fn get_input_solutions(vm: &VM<'_>) -> Result<Vec<u64>> {
    let mut solutions = Vec::new();
    for symbol in vm.parameters.iter() {
        let solution = get_single_u64_solution(&vm.solver, symbol)?;
        solutions.push(solution);
    }
    Ok(solutions)
}

fn get_sym_solutions(vm: &VM<'_>) -> Result<HashMap<String, u64>> {
    let mut solutions = HashMap::new();
    for (name, symbol) in vm.state.symbols.iter() {
        let solution = get_single_u64_solution(&vm.solver, symbol)?;
        solutions.insert(name.clone(), solution);
    }
    Ok(solutions)
}

fn get_single_u64_solution(solver: &Solver, value: &BV) -> Result<u64> {
    let solution = solver.get_solutions_for_bv(value, 1)?;

    let solution = match solution {
        Solutions::None => return Err(anyhow!("No input solutions found")),
        Solutions::Exactly(n) | Solutions::AtLeast(n) => n
            .first()
            .unwrap()
            .as_u64()
            .expect("Could not convert solution to u64"),
    };

    Ok(solution)
}

pub fn run_analysis(path: impl AsRef<Path>, fn_name: &str) -> Result<()> {
    let project = Project::from_bc_path(path)?;
    let mut vm = VM::new(fn_name, &project)?;

    let mut results = Vec::new();
    while let Some(path_result) = vm.run() {
        // Get input solutions.
        let input_solutions = get_input_solutions(&vm).expect("Could not get input solutions");

        // Get solutions to `symbolic`.
        let sym_solutions =
            get_sym_solutions(&vm).expect("Could not get solutions for `symbolic` calls");

        // Get solution to output.
        let output = match &path_result {
            Ok(v) => match v {
                ReturnValue::Value(v) => Some(
                    get_single_u64_solution(&vm.solver, v).expect("failed to get output solution"),
                ),
                ReturnValue::Void => None,
            },
            Err(_) => None,
        };

        results.push(Solution {
            ret: path_result,
            inputs: input_solutions,
            symbolic: sym_solutions,
            output,
        });
    }

    println!("Results: {results:#?}");
    Ok(())
}

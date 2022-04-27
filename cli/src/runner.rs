use anyhow::{anyhow, Result};
use colored::Colorize;
use rustc_demangle::demangle;
use std::path::Path;
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

    /// Values explicitly marked as symbolic. One solution for each of them.
    symbolic: Vec<(String, u64)>,

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

fn get_sym_solutions(vm: &VM<'_>) -> Result<Vec<(String, u64)>> {
    let mut solutions = Vec::new();
    for (name, symbol) in vm.state.symbols.iter() {
        let solution = get_single_u64_solution(&vm.solver, symbol)?;
        solutions.push((name.clone(), solution));
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
    let project = Project::from_path(path)?;
    let mut vm = VM::new(fn_name, &project)?;

    let mut results = Vec::new();
    let mut path = 1;
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

        if path != 1 {
            println!("\n");
        }
        println!(
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ PATH {path} ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        );
        match &path_result {
            Ok(_) => {
                print!("{}", "Success".green());
                let value = match output {
                    Some(value) => format!("{value}"),
                    None => "void".to_owned(),
                };
                println!(": returned {value}");
                if !sym_solutions.is_empty() {
                    println!("\nSymbolic values");
                    for (n, (name, value)) in sym_solutions.iter().enumerate() {
                        println!("{n:4}: {name}={value}");
                    }
                }
                if !input_solutions.is_empty() {
                    println!("\nInputs");
                    for (n, value) in input_solutions.iter().enumerate() {
                        println!("{n:4}: {value}");
                    }
                }
            }
            Err(error) => {
                println!("{}", format!("Error: {error}").red());
                if let Some(source_loc) = vm.state.current_loc.source_loc {
                    println!("    at {source_loc}\n");
                }
                println!("Callstack:");
                for (n, callsite) in vm.state.callstack.iter().rev().enumerate() {
                    let fn_name = demangle(callsite.location.func.name.as_str());
                    println!("{n:4}: {fn_name:#}");
                    if let Some(debug_loc) = callsite.location.source_loc {
                        println!("      at {debug_loc}");
                    }
                }
                if !sym_solutions.is_empty() {
                    println!("\nSymbolic values");
                    for (n, (name, value)) in sym_solutions.iter().enumerate() {
                        println!("{n:4}: {name}={value}");
                    }
                }
                if !input_solutions.is_empty() {
                    println!("\nInputs");
                    for (n, value) in input_solutions.iter().enumerate() {
                        println!("{n:4}: {value}");
                    }
                }
            }
        }

        results.push(Solution {
            ret: path_result,
            inputs: input_solutions,
            symbolic: sym_solutions,
            output,
        });
        path += 1;
    }

    //println!("Results: {results:#?}");
    Ok(())
}

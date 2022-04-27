use anyhow::Result;
use colored::Colorize;
use rustc_demangle::demangle;
use std::path::Path;
use x0001e::{
    project::Project,
    solver::SolutionGenerator,
    vm::{self, ReturnValue, VM},
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

pub fn run_analysis(path: impl AsRef<Path>, fn_name: &str) -> Result<()> {
    let project = Project::from_path(path)?;
    let mut vm = VM::new(fn_name, &project)?;

    let mut results = Vec::new();
    let mut path = 1;
    while let Some(path_result) = vm.run() {
        let mut gen = SolutionGenerator::new(vm.solver.clone());

        let mut parameters = Vec::new();
        for bv in vm.parameters.iter() {
            let sol = gen.get_solution(bv)?;
            parameters.push(sol.as_u64().unwrap());
        }

        let mut symbolics = Vec::new();
        for (name, bv) in vm.state.symbols.iter() {
            let sol = gen.get_solution(bv)?;
            symbolics.push((name.clone(), sol.as_u64().unwrap()));
        }

        let output = if let Ok(result) = &path_result {
            match result {
                ReturnValue::Value(v) => gen.get_solution(v)?.as_u64(),
                ReturnValue::Void => None,
            }
        } else {
            None
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
                if !symbolics.is_empty() {
                    println!("\nSymbolic values");
                    for (name, value) in symbolics.iter() {
                        println!("    {name}: {value}");
                    }
                }
                if !parameters.is_empty() {
                    println!("\nInputs");
                    for (n, value) in parameters.iter().enumerate() {
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
                if !symbolics.is_empty() {
                    println!("\nSymbolic values");
                    for (name, value) in symbolics.iter() {
                        println!("    {name}: {value}");
                    }
                }
                if !parameters.is_empty() {
                    println!("\nInputs");
                    for (n, value) in parameters.iter().enumerate() {
                        println!("{n:4}: {value}");
                    }
                }
            }
        }

        results.push(Solution {
            ret: path_result,
            inputs: parameters,
            symbolic: symbolics,
            output,
        });
        path += 1;
    }

    //println!("Results: {results:#?}");
    Ok(())
}

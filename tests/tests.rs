use anyhow::{anyhow, Result};
use x0001e::{
    memory::{MemoryError, CHECK_OUT_OF_BOUNDS},
    project::Project,
    vm::{self, ReturnValue, VMError, VM},
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

fn run(path: &str, f: &str) -> Vec<Solution> {
    let _ = env_logger::builder().is_test(true).try_init();

    let project = Project::from_path(path).expect("Failed to create project");
    let mut vm = VM::new(f, &project).expect("Failed to create VM");

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

    println!("Results: {:?}", results);
    results
}

// Check that simple instructions work.
#[test]
fn simple() {
    let res = run("tests/samples/simple.bc", "main");
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0].output, Some(0));
}

// Check that simple if statements work.
//
// The path is hardcoded so only a single path should be taken.
#[test]
fn ifs() {
    let res = run("tests/samples/ifs.bc", "main");
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0].output, Some(1));
}

// Check that if statements work.
//
// The input here is symbolic so both paths should be taken.
#[test]
fn multiple_paths() {
    let res = run("tests/samples/multiple_paths.bc", "foo");
    assert_eq!(res.len(), 2, "expected 2 paths");
    assert_eq!(res[0].output, Some(1));
    assert_eq!(res[1].output, Some(2));
}

// Check that function calls work.
//
// This also has backtracking so it ensures it correctly handles re-entry into an inner function.
#[test]
fn call() {
    let res = run("tests/samples/call.bc", "bar");
    assert_eq!(res.len(), 2, "expected 2 paths");
    assert_eq!(res[0].output, Some(10));
    assert_eq!(res[1].output, Some(1));
}

// Test that it can handle traits (global variable vtables).
#[test]
fn traits() {
    let res = run("tests/samples/traits.bc", "traits::foo");
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0].output, Some(10));
}

// Test that it handles match expressions.
#[test]
fn match_works() {
    let res = run("tests/samples/match.bc", "match::main");
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0].output, None);
}

// Check that it can handle global references to functions.
//
// This will call one of two functions depending on a bool.
#[test]
fn fn_refs() {
    let res = run("tests/samples/fn_refs.bc", "fn_refs::foo");
    assert_eq!(res.len(), 2, "expected 2 paths");
    assert_eq!(res[0].output, Some(10));
    assert_eq!(res[1].output, Some(11));
}

// Check that array indexing works.
//
// For typical checked array gets it will return panic on out of bounds.
#[test]
fn array_index() {
    let res = run("tests/samples/array_index.bc", "array_index::get");
    assert_eq!(res.len(), 2, "expected 2 paths");
    assert!(res[0].ret.is_ok());
    assert!(res[0].inputs[0] <= 3);

    assert_eq!(res[1].ret, Err(VMError::Abort(-1)));
    assert!(res[1].inputs[0] > 3);
}

// Check that array indexing works.
#[test]
fn array_index_works() {
    let res = run(
        "tests/samples/array_index.bc",
        "array_index::indexing_works",
    );
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0].output, Some(3));
}

// Check that array indexing works even when using the unsafe `get_unchecked`
#[test]
fn array_index_get_unchecked() {
    if CHECK_OUT_OF_BOUNDS {
        let res = run("tests/samples/array_index.bc", "array_index::get_unchecked");
        assert_eq!(res.len(), 1, "expected 1 path");
        assert_eq!(
            res[0].ret,
            Err(VMError::MemoryError(MemoryError::OutOfBounds))
        );
    }
}

// Check that basic loops work
#[test]
fn loops_work() {
    let res = run("tests/samples/loop.bc", "loop::simple_loop_works");
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0].output, Some(0xbc));
}

use x0001e::ExecutorError;

mod test_runner;

use test_runner::run;

// Check that simple instructions work.
#[test]
fn simple() {
    let res = run("tests/samples/simple.bc", "main");
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0], Ok(Some(0)));
}

// Check that simple if statements work.
//
// The path is hardcoded so only a single path should be taken.
#[test]
fn ifs() {
    let res = run("tests/samples/ifs.bc", "main");
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0], Ok(Some(1)));
}

// Check that if statements work.
//
// The input here is symbolic so both paths should be taken.
#[test]
fn multiple_paths() {
    let res = run("tests/samples/multiple_paths.bc", "foo");
    assert_eq!(res.len(), 2, "expected 2 paths");
    assert_eq!(res[0], Ok(Some(1)));
    assert_eq!(res[1], Ok(Some(2)));
}

// Check that function calls work.
//
// This also has backtracking so it ensures it correctly handles re-entry into an inner function.
#[test]
fn call() {
    let res = run("tests/samples/call.bc", "bar");
    assert_eq!(res.len(), 2, "expected 2 paths");
    assert_eq!(res[0], Ok(Some(10)));
    assert_eq!(res[1], Ok(Some(1)));
}

// Test that it can handle traits (global variable vtables).
#[test]
fn traits() {
    let res = run("tests/samples/traits.bc", "traits::foo");
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0], Ok(Some(10)));
}

// Test that it handles match expressions.
#[test]
fn match_works() {
    let res = run("tests/samples/match.bc", "match::main");
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0], Ok(None));
}

// Check that it can handle global references to functions.
//
// This will call one of two functions depending on a bool.
#[test]
fn fn_refs() {
    let res = run("tests/samples/fn_refs.bc", "fn_refs::foo");
    println!("res: {res:#?}");
    assert_eq!(res.len(), 2, "expected 2 paths");
    assert_eq!(res[0], Ok(Some(10)));
    assert_eq!(res[1], Ok(Some(11)));
}

// Check that array indexing works.
//
// For typical checked array gets it will return panic on out of bounds.
#[test]
fn array_index1() {
    let res = run("tests/samples/array_index.bc", "array_index::get");
    println!("res: {res:#?}");
    assert_eq!(res.len(), 2, "expected 2 paths");
    assert!(res[0].is_ok());
    // assert!(res[0].inputs[0]) <= 3);

    assert_eq!(res[1], Err(ExecutorError::Abort(-1)));
    // assert!(res[1].inputs[0]) > 3);
}

// Check that array indexing works.
#[test]
fn array_index_works() {
    let res = run(
        "tests/samples/array_index.bc",
        "array_index::indexing_works",
    );
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0], Ok(Some(3)));
}

// // Check that array indexing works even when using the unsafe `get_unchecked`
// #[test]
// fn array_index_get_unchecked() {
//     if CHECK_OUT_OF_BOUNDS {
//         let res = run("tests/samples/array_index.bc", "array_index::get_unchecked");
//         assert_eq!(res.len(), 1, "expected 1 path");
//         assert_eq!(
//             res[0],
//             Err(ExecutorError::MemoryError(MemoryError::OutOfBounds))
//         );
//     }
// }

// Check that basic loops work
#[test]
fn loops_work() {
    let res = run("tests/samples/loop.bc", "loop::simple_loop_works");
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0], Ok(Some(0xbc)));
}

//
#[test]
fn structs_work() {
    let res = run("tests/samples/structs.bc", "structs::foo");
    assert_eq!(res.len(), 1, "expected 1 path");
    assert_eq!(res[0], Ok(Some(15)));
}

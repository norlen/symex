use env_logger::init;
use x0001e::{project::Project, vm::VM};

fn main() {
    println!("init");
    init();

    let project =
        Project::from_bc_path("./target/debug/examples/rust_simple_code-22c67ec3ab21d161.bc")
            .unwrap();

    println!("here");
    let vm = VM::new("rust_simple_code::rust_simple_test", &project).unwrap();

    let mut results = Vec::new();
    for path in vm {
        println!("Path result: {:?}", path);
        results.push(path);
    }
    println!("Results: {:?}", results);
}

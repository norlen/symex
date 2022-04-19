use env_logger::init;
use x0001e::{project::Project, vm::VM};

fn main() {
    println!("init");
    init();

    let project =
        Project::from_bc_path("./target/debug/examples/rust_simple_code-fbd4e958c67e8886.bc")
            .unwrap();

    println!("here");
    let vm = VM::new("rust_simple_code::t", &project).unwrap();

    for path in vm {
        println!("Path result: {:?}", path);
    }
}

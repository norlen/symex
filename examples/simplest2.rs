use env_logger::init;
use x0001e::{
    project::Project,
    vm::{ReturnValue, VM},
};

fn main() {
    init();

    let project = Project::from_bc_path("./samples/simplest/main.bc").unwrap();
    let mut vm = VM::new("t", &project).unwrap();

    let path: Result<ReturnValue, _> = vm.run();
    println!("result {:?}", path);
}

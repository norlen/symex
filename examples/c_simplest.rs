use env_logger::init;
use x0001e::{
    project::Project,
    vm::{ReturnValue, VM},
};

fn main() {
    init();

    let project = Project::from_path("./samples/simplest/main.bc").unwrap();
    let vm = VM::new("t", &project).unwrap();

    // this creates a problem with clone
    let paths: Vec<Result<ReturnValue, _>> = vm.clone().into_iter().collect();
    println!("result {:?}", paths);

    for path in vm {
        println!("Path result: {:?}", path);
    }
}

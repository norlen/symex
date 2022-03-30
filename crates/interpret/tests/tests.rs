use breeze_bc_interp::project::Project;
use breeze_bc_interp::vm::VM;

#[test]
fn simple_test() {
    let project = Project::from_bc_path("./tests/bcs/simple.bc").unwrap();

    let mut em = VM::new("main", &project).unwrap();
    em.next().unwrap().unwrap();
}

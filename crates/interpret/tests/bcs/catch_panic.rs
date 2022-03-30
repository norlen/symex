use std::panic;

fn this_will_panic() {
    panic!();
}

fn foo() {
    let result = panic::catch_unwind(|| {
        this_will_panic();
    });
}

fn main() {
    foo();
}

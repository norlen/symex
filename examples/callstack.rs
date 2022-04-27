//! Run example with `cargo run -p cli -- --example callstack --function foobar`
use x0001e::{assume, symbolic};

fn bar(x: i32, y: i32) -> i32 {
    // Assume these are not too big, otherwise it catches overflow as well (at least in debug)
    assume(x < 1000);
    assume(y < 1000);
    if x + y == 10 {
        panic!();
    } else {
        x
    }
}

fn foo(x: i32) -> i32 {
    let mut y = 0;
    if x > 100 {
        symbolic(&mut y);
        bar(x, y)
    } else {
        0
    }
}

fn foobar(x: i32) -> i32 {
    foo(x)
}

fn main() {
    foobar(1);
}

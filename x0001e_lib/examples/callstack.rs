//! Run example with `cargo run -p cli -- --example callstack --function foobar`
use x0001e_lib::{assume, symbolic};

fn bar(x: i64, y: i64) -> i64 {
    // Assume these are not too big, otherwise it catches overflow as well (at least in debug)

    assume(x < 1000 && x > 0);
    assume(y < 1000 && x > 0);

    if x + y == 10 {
        panic!();
    } else {
        x
    }
}

fn foo(x: i64) -> i64 {
    let mut y = 0;
    if x > 100 {
        symbolic(&mut y);
        bar(x, y)
    } else {
        0
    }
}

fn foobar(x: i64) -> i64 {
    foo(x)
}

fn main() {
    foobar(1);
}

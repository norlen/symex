use x0001e::{assume, symbolic};

fn rust_simple_test(t: u32) -> u32 {
    assume(t > 0);
    if t == 0 {
        // This path should never happen.
        panic!("does not work");
    }
    symbolic(&t);
    if t == 0 {
        // But this should.
        0
    } else {
        11
    }
}

fn main() {
    rust_simple_test(6);
}

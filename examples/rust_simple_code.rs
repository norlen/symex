use x0001e::{assume, symbolic};

fn rust_simple_test(t: u32) -> u32 {
    assume(t > 0);
    symbolic(&t);
    if t > 10 {
        0
    } else {
        11
    }
}

fn main() {
    rust_simple_test(6);
}

use x0001e::assume;

fn t(t: u32) -> u32 {
    if t > 10 {
        0
    } else {
        10
    }
}

fn main() {
    t(6);
}

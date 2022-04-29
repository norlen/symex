fn simple_loop_works() -> i32 {
    for i in 0..10 {
        if i == 5 {
            return 0xbc;
        }
    }
    0
}

fn loops(x: i32) -> i32 {
    for i in 0..x {
        if i == 5 {
            return 1;
        }
    }
    0
}

fn main() {
    simple_loop_works();
    loops(1);
}

#![allow(dead_code)]

fn function_to_analyze(x: u32) -> u32 {
    if x < 10 {
        x
    } else {
        10
    }
}

fn my_func(x: u32) -> u32 {
    x
}

fn main() {
    function_to_analyze(0);
    my_func(0);
}

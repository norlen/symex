#[inline(never)]
pub fn foo(x: i32, y: i32) -> i32 {
    if x > 5 && x + y == 100 {
        if x * y == 1875 {
            panic!();
        } else {
            5
        }
    } else {
        //3
        x / y
    }
}

fn main() {
    println!("{}", foo(1, 1));
}

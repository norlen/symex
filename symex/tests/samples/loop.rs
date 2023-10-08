#![allow(dead_code)]

#[no_mangle]
fn symex_symbolic(_: *mut std::ffi::c_void, _: u64) {}
fn symbolic<T>(value: &mut T) {
    unsafe {
        let size = std::mem::size_of_val(value);
        let ptr = std::mem::transmute(value);
        symex_symbolic(ptr, size as u64);
    }
}

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

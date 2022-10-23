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

fn myfunc(a: i32) -> i32 {
    a
}

fn myfunc2(a: i32) -> i32 {
    a + 1
}

fn bar() -> fn(i32) -> i32 {
    myfunc
}

fn bar2() -> fn(i32) -> i32 {
    myfunc2
}

fn foo(b: bool) -> i32 {
    let f = if b { bar() } else { bar2() };
    let a = 10;
    let b = f(a);
    b
}

fn callme() -> i32 {
    let mut b = false;
    symbolic(&mut b);
    foo(b)
}

fn main() {}

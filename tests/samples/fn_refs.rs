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

fn main() {
    foo(true);
}

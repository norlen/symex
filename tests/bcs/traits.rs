trait Foo {
    fn get_random(&mut self) -> i32;
}

struct Bar {
    n: i32,
}

impl Foo for Bar {
    fn get_random(&mut self) -> i32 {
        self.n += 5;
        self.n
    }
}

fn rand(f: &mut dyn Foo) -> i32 {
    f.get_random()
}

fn foo() -> i32 {
    let mut b = Bar { n: 5 };
    rand(&mut b)
}

fn main() {
    let mut b = Bar { n: 5 };
    rand(&mut b);
    let z = foo();
}

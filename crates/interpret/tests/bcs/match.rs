enum Shape {
    Square(i32),
    Rectangle(i32, i32),
}

impl Shape {
    fn area(&self) -> i32 {
        match self {
            Shape::Square(w) => w * w,
            Shape::Rectangle(w, h) => w * h,
        }
    }
}

fn main() {
    let s = Shape::Square(5);
    let z = s.area();
}

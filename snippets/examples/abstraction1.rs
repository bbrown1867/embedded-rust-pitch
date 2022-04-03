use std::ops::Mul;

struct Point<T> {
    x: T,
    y: T,
}

impl<T: Mul<Output = T> + Copy> Point<T> {
    fn product(&self) -> T {
        self.x * self.y
    }
}

fn main() {
    let p = Point { x: 5.0, y: 10.0 };
    println!("x * y = {}", p.product());
}

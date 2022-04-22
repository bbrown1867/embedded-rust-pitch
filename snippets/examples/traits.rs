use std::ops::Mul;

struct Point<T> {
    x: T,
    y: T,
}

impl<T: Mul<Output = T> + Copy> Mul for Point<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1 * p2;
    println!("x = {}, y = {}", p3.x, p3.y);
}

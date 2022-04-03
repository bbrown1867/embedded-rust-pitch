struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<i32> {
    fn product(&self) -> i32 {
        &self.x * &self.y
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("x = {}, y = {}", p.x(), p.y());
    println!("x * y = {}", p.product());
}

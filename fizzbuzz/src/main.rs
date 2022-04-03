fn fizz_buzz(limit: u32) {
    for i in 1..limit {
       match (i % 3, i % 5) {
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            (_, _) => println!("{}", i),
       }
    }
}

fn main() {
    let limit = 101;
    fizz_buzz(limit);
}

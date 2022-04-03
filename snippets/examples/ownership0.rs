//! Simple ownership example: Moving a value.
//! Reference: <https://www.youtube.com/watch?v=wXoY91w4Agk>

fn give() {
    // `give` is the owner of vec
    let mut vec = Vec::new();
    vec.push(42);

    // `take` is the owner of vec
    take(vec);

    // error[E0382]: borrow of moved value: `vec`
    println!("Size = {}", vec.len());
}

fn take(vec: Vec<i32>) {
    println!("Size = {}", vec.len());

    // `vec` is freed here
}

fn main() {
    give()
}

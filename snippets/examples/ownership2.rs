//! Simple ownership example: Mutable references.
//! Reference: <https://www.youtube.com/watch?v=wXoY91w4Agk>

fn push_all(from: &Vec<i32>, to: &mut Vec<i32>) {
    for elem in from {
        to.push(*elem);
    }
}

fn main() {
    let mut vec = vec![1, 2, 3, 4];
    let mut vec2 = Vec::new();
    push_all(&vec, &mut vec2);

    println!("Size = {}", vec.len());
    println!("Size = {}", vec2.len());

    // error[E0502]: cannot borrow `vec` as mutable
    // because it is also borrowed as immutable
    push_all(&vec, &mut vec);
}

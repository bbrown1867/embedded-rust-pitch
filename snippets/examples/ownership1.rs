//! Simple ownership example: Shared references.
//! Reference: <https://www.youtube.com/watch?v=wXoY91w4Agk>

fn lender() {
    let mut vec = Vec::new();
    vec.push(42);

    borrower(&vec);

    println!("Size = {}", vec.len());
}

fn borrower(vec: &Vec<i32>) {
    println!("Size = {}", vec.len());

    // error[E0596]: cannot borrow `*vec` as mutable
    vec.push(43);
}

fn main() {
    lender()
}

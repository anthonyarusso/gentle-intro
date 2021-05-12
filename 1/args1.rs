use std::env;

fn main() {
    let first = env::args().nth(2).expect("Please supply an argument");
    let n: i32 = first.parse().expect("Not an integer!");

    println!("n {}", n);
}

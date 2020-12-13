use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize
    }

    println!("{}", min(min(a1, a2), min(a3, a4)));
}
use proconio::input;
use std::cmp::{min, max};

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    if min(x, y) + 3 > max(x, y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut min_cost = std::i32::MAX;

    for i in -100..=100 {
        let sum = a.iter().fold(0, |sum, x| sum + (x - i) * (x - i));
        min_cost = min(min_cost, sum);
    }

    println!("{}", min_cost);
}
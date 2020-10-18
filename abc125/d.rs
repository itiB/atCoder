use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut negative = 0;
    let mut sum = 0;
    let mut minimal = 1_000_000_000;

    for num in a {
        sum += num.abs();
        if num < 0 {
            negative += 1;
        }
        minimal = min(minimal, num.abs());
    }
    println!("{}", if negative % 2 == 1 {
        sum - minimal - minimal
    } else { sum });
}

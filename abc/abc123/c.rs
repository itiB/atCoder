use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: i64,
        a: [i64; 5],
    }

    let mut m = 1_000_000_000_000_001;
    for i in a {
        m = min(m, i);
    }
    println!("{}", 4 + (n + m - 1)/m);
}
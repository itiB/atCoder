use proconio::input;
use std::cmp::{min, max};

fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize
    }

    let mut mikan_min = 1_000_000_000;
    let mut mikan_max = 0;

    for n in 1..=1_000_000 {
        if a * n <= 1000 * w && 1000 * w <= b * n {
            mikan_min = min(mikan_min, n);
            mikan_max = max(mikan_max, n);
        }
    }
    if mikan_max == 0 {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", mikan_min, mikan_max);
    }
}

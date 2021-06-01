use proconio::input;
use std::cmp::{min, max};

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }
    let mut ans = 1_000_000_000;

    for i in 0..n {
        for j in 0..n {
            ans = min(ans, if i == j { ab[i].0 + ab[j].1 } else {max(ab[i].0, ab[j].1)});
        }
    }
    println!("{}", ans);
}

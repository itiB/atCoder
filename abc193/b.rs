use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        apx: [(i64, i64, i64); n]
    }

    let mut ans = 1_000_000_001;

    for (a, p, x) in apx {
        if x > a {
            ans = min(ans, p);
        }
    }
    if ans == 1_000_000_001 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
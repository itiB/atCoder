use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        a1: i128,
        a2: i128,
        a3: i128,
    }

    let mut ans: i128 = 1_000_000_000_000_000_001;
    // a1, a3 を変えるとき
    if a2 + a2 - a1 - a3 >= 0 {
        ans = min(ans, a2 + a2 - a1 - a3);
    }
    // a2 を変えるとき
    if a3 - a2 - a2 + a1 >= 0 {
        if (a3 - a2 - a2 + a1) % 2 == 1 {
            ans = min(ans, (a3 - a2 - a2 + a1 + 1) / 2 + 1);
        } else {
            ans = min(ans, (a3 - a2 - a2 + a1) / 2);
        }
    }
    println!("{}", ans);
}
use proconio::input;
use std::cmp::min;
const MAX: i64 = 1_000_000_001;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n]
    }

    let mut dp = vec![MAX; n];
    dp[0] = 0;
    for i in 1..n {
        for j in 0..=k {
            if i >= j {
                dp[i] = min(dp[i], dp[i-j]+(h[i] - h[i-j]).abs());
            }
        }
    }
    println!("{}", dp[n-1]);
}
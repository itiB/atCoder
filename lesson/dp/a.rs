use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        h: [i64; n]
    }

    let mut dp = vec![0; n];
    dp[1] = (h[1] - h[0]).abs();
    for i in 2..n {
        dp[i] = min(dp[i-1]+(h[i] - h[i-1]).abs(), dp[i-2]+(h[i] - h[i-2]).abs());
    }
    println!("{}", dp[n-1]);
}
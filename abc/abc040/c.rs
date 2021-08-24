use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut dp = vec![0; n];
    dp[1] = (a[1] - a[0]).abs();
    for i in 2..n {
        dp[i] = min(dp[i - 1] + (a[i] - a[i-1]).abs(), dp[i - 2] + (a[i] - a[i-2]).abs());
    }
    println!("{}", dp[n - 1]);
}
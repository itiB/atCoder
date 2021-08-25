use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m]
    }

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..=n { dp[i][0] = i }
    for i in 0..=m { dp[0][i] = i }
    for i in 1..=n {
        for j in 1..=m {
            let cost = if a[i-1] != b[j-1] { 1 } else { 0 };
            dp[i][j] = min(dp[i-1][j]+1, min(dp[i][j-1]+1, dp[i-1][j-1]+cost));
        }
    }
    println!("{}", dp[n][m])
}
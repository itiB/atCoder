use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, usize); n]
    }

    let mut dp = vec![vec![0; n]; 3];
    dp[0][0] = abc[0].0;
    dp[1][0] = abc[0].1;
    dp[2][0] = abc[0].2;
    for i in 1..n {
        dp[0][i] = abc[i].0 + max(dp[1][i-1], dp[2][i-1]);
        dp[1][i] = abc[i].1 + max(dp[0][i-1], dp[2][i-1]);
        dp[2][i] = abc[i].2 + max(dp[0][i-1], dp[1][i-1]);
    }
    println!("{}", max(dp[0][n-1], max(dp[1][n-1], dp[2][n-1])));
}
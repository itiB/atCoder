use proconio::input;
use std::cmp::{min, max};

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
        y: usize,
    }

    let mut dp = vec![vec![std::i32::MAX as usize; 101]; 2];
    // まず縦だけにAを上る
    dp[0][a] = 0;
    for i in a + 1..101 {
        dp[0][i] = dp[0][i - 1] + y;
    }
    for i in (0..a).rev() {
        dp[0][i] = dp[0][i + 1] + y;
    }

    // ただBに乗り移るだけの時
    for i in 0..100 {
        dp[1][i] = min(dp[0][i], dp[0][i + 1]) + x;
    }
    dp[1][100] = min(dp[0][100] + x, dp[1][99] + y);

    for i in a + 1..101 {
        dp[0][i] = min(dp[0][i], dp[1][i - 1] + x);
        dp[1][i] = min(dp[1][i], dp[0][i] + x);
    }
    for i in (0..a).rev() {
        dp[0][i] = min(dp[0][i], dp[1][i + 1] + x);
        dp[1][i] = min(dp[1][i], dp[0][i] + x);
    }


    // println!("{:?}", dp);
    println!("{}", dp[1][b]);
}
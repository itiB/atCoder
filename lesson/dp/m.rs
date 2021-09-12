use proconio::input;
use std::cmp::max;
const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n]
    }

    let mut dp = vec![vec![0; k+1]; n+1];
    // 0人目は必ず0個を持つ通りが1つだけある
    dp[0][0] = 1;
    let mut su = vec![vec![0; k+2]; n+1];
    for i in 1..=n {
        // 1つ前の行の累積和を作る。
        for j in 1..=k+1 {
            su[i-1][j] = (dp[i-1][j-1] + su[i-1][j-1]) % MOD;
        }
        // dpの更新、j-a_i-1 ~ jの範囲の合計を累積和から調べる
        for j in 0..=k {
            let t = j as i64 - a[i-1];
            dp[i][j] = (MOD + su[i-1][j+1] - su[i-1][max(0, t) as usize]) % MOD;
        }
    }
    println!("{}", dp[n][k]);
}
use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 0..n {
        for j in 0..=w {
            if j >= wv[i].0 {
                // 現在の荷物量jの中で最大の価値を生み出すものをdp[i][j]に入れる
                // 1, 選ばなかった時: dp[i-1][j]の価値
                // 2. 選んだとき    : dp[i-1][j - 今から増やす重さの位置] + 増やす価値
                dp[i + 1][j] = max(dp[i][j], dp[i][j - wv[i].0] + wv[i].1);
            } else {
                // 荷物が重すぎて入らない場合は左隣の価値をそのまま引きつぐ
                dp[i + 1][j] = dp[i][j];
            }
        }
    }
    println!("{}", dp[n][w]);
}

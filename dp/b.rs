use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i32; n],
    }

    println!("{}", dp(n, k, &h));
}

fn dp(n: usize, k: usize, h: &[i32]) -> usize {
    let mut dp = vec![0; n];

    for i in 1..n {
        // dpを求めるスタート地点を設定
        let start = if i <= k { 0 } else { i - k };

        let mut minimal = std::i32::MAX;
        for j in start..i {
            // カエルが飛べるなかでdpが最も低くなるところを探す
            minimal = min(minimal, dp[j] + (h[i] - h[j]).abs());
        }
        dp[i] = minimal;
    }
    dp[n - 1] as usize
}
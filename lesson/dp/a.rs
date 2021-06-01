use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }

    let mut dp = vec![0; n];

    for i in 1..n {
        if i > 1 {
            // i-1 か i-2 からしか来ることはない
            dp[i] = min(dp[i - 1] + (h[i - 1] - h[i]).abs(), 
                        dp[i - 2] + (h[i - 2] - h[i]).abs())
        } else {
            // i-2 < 1 のときに i-2 はないので i-1のみ
            dp[i] = dp[i - 1] + (h[i - 1] - h[i]).abs();
        }
    }
    println!("{}", dp[n - 1]);
}

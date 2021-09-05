use proconio::input;
use std::cmp::{min, max};

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut dp = vec![vec![0; n+1]; n+1];
    for width in 1..=n {
        for l in 0..=n - width {
            let r = l + width;
            if width % 2 == n % 2 {
                // Taro できる限り大きくなるように取っていく
                dp[l][r] = max(dp[l+1][r] + a[l], dp[l][r-1] + a[r-1]);
            } else {
                // Jiro できる限り多く引けるほうを取っていく
                dp[l][r] = min(dp[l+1][r] - a[l], dp[l][r-1] - a[r-1]);
            }
        }
    }
    println!("{}", dp[0][n]);
}
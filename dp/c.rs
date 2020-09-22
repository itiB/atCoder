use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, usize); n],
    }

    let mut dp = vec![(0, 0, 0); n + 1];
    for i in 1..=n {
        dp[i].0 = max(dp[i - 1].1, dp[i - 1].2) + abc[i - 1].0;
        dp[i].1 = max(dp[i - 1].0, dp[i - 1].2) + abc[i - 1].1;
        dp[i].2 = max(dp[i - 1].0, dp[i - 1].1) + abc[i - 1].2;
    }

    println!("{}", max(dp[n].0, max(dp[n].1, dp[n].2)));
}

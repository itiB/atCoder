use proconio::input;
use std::cmp::min;
const RANGE: usize = 100_001;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n]
    }

    let mut dp = vec![vec![None; RANGE + 1]; n+1];
    dp[0][0] = Some(0);
    for i in 1..=n {
        for j in (0..=RANGE).rev() {
            if let Some(v) = dp[i-1][j] {
                dp[i][j] = Some(v);
                let tmp = dp[i][j].unwrap() + wv[i-1].0;
                dp[i][j + wv[i - 1].1] = match dp[i][j + wv[i - 1].1] {
                    None => Some(tmp),
                    Some(j) => Some(min(j, tmp)),
                }
            }
        }
    }
    for v in (0..=RANGE).rev() {
        if let Some(x) = dp[n][v] {
            if x <= w {
                println!("{}", v);
                return
            }
        }
    }
}
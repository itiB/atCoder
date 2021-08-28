use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n]
    }

    let mut dp = vec![vec![None; w+1]; n+1];
    dp[0][0] = Some(0);
    for i in 1..=n {
        for j in (0..=w).rev() {
            if let Some(v) = dp[i-1][j] {
                dp[i][j] = Some(v);
                if j + wv[i - 1].0 <= w {
                    let tmp = dp[i][j].unwrap() + wv[i-1].1;
                    dp[i][j + wv[i - 1].0] = match dp[i][j + wv[i - 1].0] {
                        None => Some(tmp),
                        Some(j) => Some(max(j, tmp)),
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for i in &dp[n] {
        if let Some(v) = i {
            ans = max(ans, *v);
        }
    }
    println!("{}", ans);
}
use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

const MOD: usize = 1_000_000_007;
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut dp = vec![vec![0; w+1]; h+1];
    dp[1][1] = 1;
    for y in 1..=h {
        for x in 1..=w {
            if a[y-1][x-1] == '.' {
                dp[y][x] = max(dp[y][x], (dp[y-1][x] + dp[y][x-1]) % MOD);
            }
        }
    }
    println!("{}", dp[h][w]);
}

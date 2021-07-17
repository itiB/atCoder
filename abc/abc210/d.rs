use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: usize,
        a: [[usize; w]; h],
    }

    let mut ans: usize = 1_000_000_000_000;
    // 1つ目の南東に2つ目の駅があるとする
    // 1つ目の駅は建てるとしてそこにたどりつくまでの金額
    let mut dp: Vec<Vec<usize>> = vec![vec![0; w+1]; h+1];
    for i in 0..=w {
        dp[0][i] = 1_000_000_000_000;
    }
    for i in 0..=h {
        dp[i][0] = 1_000_000_000_000;
    }
    for x in 1..=w {
        for y in 1..=h {
            dp[y][x] = min(a[y-1][x-1], dp[y-1][x] + c);
            dp[y][x] = min(dp[y][x], dp[y][x-1] + c);
        }
    }
    // 2つ目の駅を建ててどこが一番安くできるか調べる
    for x in 1..=w {
        for y in 1..=h {
            // 左から来るパターン
            ans = min(ans, dp[y][x-1] + c + a[y-1][x-1]);
            // 上から来るパターン
            ans = min(ans, dp[y-1][x] + c + a[y-1][x-1]);
        }
    }

    // 1つ目の南西に2つ目の駅があるとする
    let mut dp: Vec<Vec<usize>> = vec![vec![0; w+1]; h+1];
    for i in 0..=w {
        dp[0][i] = 1_000_000_000_000;
    }
    for i in 0..=h {
        dp[i][w] = 1_000_000_000_000;
    }
    for x in (0..w).rev() {
        for y in 1..=h {
            dp[y][x] = min(a[y-1][x], dp[y-1][x] + c);
            dp[y][x] = min(dp[y][x], dp[y][x+1] + c);
        }
    }
    // 2つ目の駅を建ててどこが一番安くできるか調べる
    for x in 0..w {
        for y in 1..=h {
            // 右から来るパターン
            ans = min(ans, dp[y][x+1] + c + a[y-1][x]);
            // 上から来るパターン
            ans = min(ans, dp[y-1][x] + c + a[y-1][x]);
        }
    }
    println!("{}", ans);
}
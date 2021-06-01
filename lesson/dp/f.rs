use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![0; s.len() + 1]; t.len() + 1];

    for y in 1..=t.len() {
        for x in 1..=s.len() {
            if s[x - 1] == t[y - 1] {
                dp[y][x] = dp[y - 1][x - 1] + 1;
            } else {
                dp[y][x] = max(dp[y][x - 1], dp[y - 1][x]);
            }
        }
    }
    let mut x = s.len();
    let mut y = t.len();
    let mut l = dp[y][x];
    let mut ans = Vec::new();
    while l > 0 {
        // 逆にたどって文字列を調べる
        if s[x - 1] == t[y - 1] {
            ans.push(s[x - 1]);
            x -= 1;
            y -= 1;
            l -= 1;
        } else if dp[y][x] == dp[y][x - 1] {
            x -= 1;
        } else {
            y -= 1;
        }
    }
    let s: String = ans.into_iter().rev().collect();
    println!("{}", s);
}

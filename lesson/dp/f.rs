use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![0; s.len()+1]; t.len()+1];
    for j in 1..=t.len() {
        for i in 1..=s.len() {
                if s[i-1] == t[j-1] {
                dp[j][i] += dp[j-1][i-1] + 1;
            } else {
                dp[j][i] = max(dp[j-1][i], dp[j][i-1]);
            }
        }
    }

    let mut l = dp[t.len()][s.len()];
    let mut ans = vec![None; l+1];
    let mut i = s.len();
    let mut j = t.len();
    while l > 0 {
        if s[i - 1] == t[j - 1] {
            ans[l] = Some(s[i-1]);
            l -= 1;
            i -= 1;
            j -= 1;
        } else if dp[j][i] == dp[j][i-1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    for c in ans {
        if let Some(v) = c {
            print!("{}", v);
        }
    }
    println!("");
}
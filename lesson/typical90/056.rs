use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n]
    }

    let mut dp = vec![vec![false; s+1]; n+1];
    // let mut added = vec!['c'; s+1];
    let mut ans = VecDeque::new();
    dp[0][0] = true;
    for day in 0..n {
        for i in 0..=s {
            if dp[day][i] {
                if i + ab[day].0 <= s {
                    dp[day + 1][i + ab[day].0] = true;
                    // added[i + ab[day].0] = 'A'
                }
                if i + ab[day].1 <= s {
                    dp[day + 1][i + ab[day].1] = true;
                    // added[i + ab[day].1] = 'B'
                }
            }
        }
    }
    if !dp[n][s] {
        println!("Impossible");
    } else {
        let mut cu = s;
        for day in (0..n).rev() {
            if cu >= ab[day].0 && dp[day][cu - ab[day].0] {
                ans.push_front('A');
                cu -= ab[day].0;
            } else {
                ans.push_front('B');
                cu -= ab[day].1;
            }
        }
        let aa: String = ans.iter().collect();
        println!("{}", aa);
    }
}
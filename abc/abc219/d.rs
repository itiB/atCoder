use proconio::input;
use std::cmp::min;

const MAX: usize = 10000;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        ab: [(usize, usize); n]
    }

    let mut ans = 301;
    let mut dp = vec![vec![301; MAX]; MAX];
    dp[0][0] = 0;
    let mut able = Vec::new();
    able.push((0, 0));
    for (a, b) in ab {
        let mut tmp = Vec::new();
        for (i, j) in &able {
            let t_x = a + i;
            let t_y = b + j;
            dp[t_x][t_y] = min(dp[*i][*j] + 1, dp[t_x][t_y]);
            if t_x >= x && t_y >= y {
                ans = min(ans, dp[t_x][t_y]);
            }
            tmp.push((t_x, t_y));
        }
        for t in tmp {
            able.push(t);
        }
    }
    println!("{}", if ans > 300 { -1 } else {ans});
}
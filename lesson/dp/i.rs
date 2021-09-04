use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [f64; n]
    }
    let mut dp = vec![vec![0.0; n+1]; n+1];
    dp[0][0] = 1.0;
    for i in 1..=n {
        dp[0][i] = dp[0][i-1] * (1.0 - p[i-1]);
    }

    for y in 1..=n { // 表の個数
        for x in 1..=n { // 投げた回数
            dp[y][x] = dp[y-1][x-1] * p[x-1] + dp[y][x-1] * (1.0 - p[x-1]);
        }
    }
    let mut ans = 0.0;
    for i in 1..=n {
        if i > n-i {
            ans += dp[i][n];
        }
    }
    println!("{}", ans);
}
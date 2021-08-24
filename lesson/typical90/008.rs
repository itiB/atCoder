use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let target: Vec<char> = "atcoder".chars().collect();

    // i文字目を選ぶか選ばないかのDPを作る
    let mut dp = vec![vec![0; n+1]; target.len() + 1];
    for x in 0..n+1 {dp[0][x] = 1;}

    for y in 1..=target.len() {
        for x in 1..n+1 {
            dp[y][x] = dp[y][x - 1];
            if s[x-1] == target[y-1] {
                dp[y][x] += dp[y-1][x-1];
            }
            dp[y][x] %= MOD;
        }
    }
    println!("{}", dp[target.len()][n]);
}
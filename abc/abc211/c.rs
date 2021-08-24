use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        s: Chars
    }

    let chokudai: Vec<char> = "chokudai".chars().collect();

    // i文字目を選ぶか選ばないかのDPを作る
    let mut dp = vec![vec![0; s.len()+1]; chokudai.len() + 1];
    for x in 0..s.len()+1 {dp[0][x] = 1;}

    for y in 1..=chokudai.len() {
        for x in 1..s.len()+1 {
            dp[y][x] = dp[y][x - 1];
            if s[x-1] == chokudai[y-1] {
                dp[y][x] += dp[y-1][x-1];
            }
            dp[y][x] %= MOD;
        }
    }

    println!("{}", dp[chokudai.len()][s.len()]);
}
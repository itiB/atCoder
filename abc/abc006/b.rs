use proconio::input;

const MOD: usize = 10007;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![0; n];
    if n > 2 { dp[2] = 1; }
    for i in 3..n {
        dp[i] = (dp[i-1] + dp[i-2] + dp[i-3]) % MOD;
    }
    println!("{}", dp[n-1]);
}
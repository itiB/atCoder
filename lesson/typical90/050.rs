use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        l: usize,
    }

    let mut dp = vec![1; n + 1];
    for i in l..=n {
        dp[i] = dp[i - 1] % MOD + dp[i - l] % MOD;
        dp[i] %= MOD;
    }
    println!("{}", dp[n]);
}
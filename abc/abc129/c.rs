use proconio::input;
use std::collections::HashSet;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }
    let a_set: HashSet<usize> = a.into_iter().collect();
    let mut dp = vec![0; n+1];

    if !a_set.contains(&0) { dp[0] = 1; }
    if !a_set.contains(&1) { dp[1] = 1; }

    for i in 2..=n {
        dp[i] = (dp[i - 1] + dp[i - 2]) % MOD;
        if a_set.contains(&i) { dp[i] = 0; }
    }
    println!("{}", dp[n]);
}
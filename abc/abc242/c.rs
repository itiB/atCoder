use proconio::input;
const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize
    }

    let mut dp = vec![1; 9];
    for i in 1..n {
        let mut tmp = vec![0; 9];
        tmp[0] = (dp[0] + dp[1]) % MOD;
        for i in 1..8 {
            tmp[i] = (dp[i-1] + dp[i] + dp[i+1]) % MOD;
        }
        tmp[8] = (dp[7] + dp[8]) % MOD;
        dp = tmp;
    }
    let ans: usize = dp.iter().sum();
    println!("{}", ans % MOD);
}
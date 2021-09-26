use proconio::input;
const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut dp = vec![vec![0; 10]; n];
    dp[1][(a[0]*a[1])%10] += 1;
    dp[1][(a[0]+a[1])%10] += 1;
    for i in 2..n {
        for j in 0..10 {
            if dp[i-1][j] > 0 {
                dp[i][(a[i]*j)%10] = (dp[i][(a[i]*j)%10] + dp[i-1][j]) % MOD;
                dp[i][(a[i]+j)%10] = (dp[i][(a[i]+j)%10] + dp[i-1][j]) % MOD;
            }
        }
    }
    for i in &dp[n-1] {
        println!("{}",i);
    }
}
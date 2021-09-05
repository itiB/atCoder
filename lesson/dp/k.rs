use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    // どうやっても先手が到達できない場所をさがす
    let mut dp = vec![false; k+1];
    for i in 1..=k {
        for j in &a {
            if i >= *j { dp[i] = dp[i] || !dp[i - j]; }
        }
    }
    println!("{}", if dp[k] { "First" } else { "Second" });
}
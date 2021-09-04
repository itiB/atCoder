use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut dp = vec![vec![vec![0.0; 303]; 303]; 303];
    let mut c = vec![0; 3];
    for i in a {
        c[i-1] += 1;
    }
    // c1,c2,c3枚の時に食べきるために振るさいころの期待値
    for c3 in 0..=n {
        for c2 in 0..=n {
            for c1 in 0..=n {
                let sum = (c1 + c2 + c3) as f64;
                if sum == 0.0 { continue; }
                // お皿に寿司が乗ってる皿をさいころ振って当てる期待値
                dp[c1][c2][c3] = n as f64 / sum;
                // つぎに起こる状態の確率を合計する
                // 1つ乗った皿をたべるパターン
                if c1 > 0 { dp[c1][c2][c3] += dp[c1-1][c2][c3] * c1 as f64/sum; }
                // 2つ乗った皿をたべるパターン
                if c2 > 0 { dp[c1][c2][c3] += dp[c1+1][c2-1][c3] * c2 as f64/sum; }
                // 3つ乗った皿をたべるパターン
                if c3 > 0 { dp[c1][c2][c3] += dp[c1][c2+1][c3-1] * c3 as f64/sum; }
            }
        }
    }
    println!("{}", dp[c[0]][c[1]][c[2]]);
}
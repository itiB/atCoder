use proconio::input;
const MAX: usize = 100 * 1000 + 1;

fn main() {
    input! {
        n: usize,
        t: [usize; n]
    }

    let mut dp = vec![false; MAX];
    dp[0] = true;
    let mut sum = 0;
    for time in t {
        sum += time;
        for i in (0..MAX - 1).rev() {
            // tまでの料理でできる合計時間を見つける
            if dp[i] { dp[i + time] = true; }
        }
    }
    for i in (sum + 1) / 2..MAX - 1 {
        // 半分以上の中からもっともすぐのTrueを探す
        if dp[i] {
            println!("{}", i);
            return
        }
    }
}
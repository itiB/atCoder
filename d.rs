use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[i64; n]; n],
    }

    let mut wa: i64 = -1;
    let mut ac: i64 = 100010001001;
    let l = k * k / 2 + 1;

    // 2分探索で調べる
    while wa + 1 < ac {
        let mut ok = false;
        let wj: i64 = (wa + ac) / 2;
        {
            // 累積和をとる
            let mut s = vec![vec![0; n+1]; n+1];
            for i in 0..n {
                for j in 0..n {
                    s[i + 1][j + 1] = if a[i][j] > wj { 1 } else { 0 };
                }
            }
            // 横に足していく
            for i in 0..n+1 {
                for j in 0..n {
                    s[i][j + 1] += s[i][j];
                }
            }
            // たてに足していく
            for i in 0..n {
                for j in 0..n+1 {
                    s[i + 1][j] += s[i][j];
                }
            }
            // 累積和から範囲内の合計値を求める
            for i in 0..n-k+1 {
                for j in 0..n-k+1 {
                    let mut now = s[i + k][j + k]; // 右上を足す
                    now -= s[i][j + k]; // 不要な部分を切る
                    now -= s[i + k][j];
                    now += s[i][j]; // 2回ひいた左上を足す
                    if now < l {
                        ok = true;
                    }
                }
            }
        }
        if ok {
            ac = wj;
        } else {
            wa = wj;
        }
    }
    println!("{}", ac);
}

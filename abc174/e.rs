use proconio::input;

// 一番長いものを切っていく
// 切った木どうしが同じ長さになるように！
// 答えを2分探索する
// f(x) = 丸太をx以下にできるかどうか
// x を 3 として
// 丸太の長さ 1 2 3 4 5 6 7 8 9
// 切る回数   0 0 0 1 1 1 2 2 2
//  -> xより大きい丸太ができない
//  -> 全部をx以下にするために切る回数 <= k ならばOK
//  -> (a_i - 1) / x

fn main() {
    input! {
        n: i64,
        k: i64,
        a: [i64; n],
    }
    let mut l = 0;
    let mut r = 1_000_000_000;
    // いくつの長さで切れば条件に当てはまるか2文探索
    while r - l > 1 {
        let mid = (l + r) / 2;
        if judge(n, mid, k, &a) {
            r = mid;
        } else {
            l = mid;
        }
    }
    println!("{}", r);
}

// 各木をx以下にするとき、切る回数がk以下になるかどうか
fn judge(n: i64, x: i64, k: i64, a: &[i64]) -> bool {
    let mut now: i64 = 0;
    for i in 0..n {
        now += (a[i as usize] - 1) / x;
    }
    now <= k
}
use proconio::input;

// はじめにxは絶対値をとって共通して正にいることにする
// 1. 往復にたどり着く前にKを使い切る
//   - 答え: x - d * k
//   - 制約: x - d * k > 0, d * k でオーバーフロー
//      - x >= dk -> x / d > kで解決！
// 2. 往復する
//   - 往復にたどり着くまで: x / d
//   - その後 -d, +d を繰り返す( k - x / d = k'回 -> 偶数or奇数)

fn main() {
    input! {
        mut x: i64,
        mut k: i64,
        d: i64,
    }

    x = x.abs();

    if (x/d > k) {
        // 往復にたどり着くまでにKを使い切る
        println!("{}", x - d * k);
    } else {
        // 往復し始めるのはe回目
        let e = x / d;
        k -= e;
        x %= d;
        if k % 2 == 1 {
            x = (x - d).abs();
        }
        println!("{}", x);
    }
}

use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize, // 1 <= N
        m: usize, // M <= 12
        x: i32, // 10 ^ 5
        c: [[i32; m + 1]; n],
    }
    let mut min_cost = i32::max_value(); // 適当に大きな数字

    // 2進数で各本の有無についてループを回す
    // ex. 0 0 0: 買わない 買わない 買わない
    //     0 0 1: 買わない 買わない 買う
    //     0 1 0: 買わない 買う 買わない
    for s in 0..1 << n {
        let mut cost = 0;
        let mut und = vec![0; m]; // 理解度

        // 各本について買ったか買ってないかをループで見る
        for i in 0..n {
            if s >> i & 1 == 1 {
                // ibit目が立っているならi番目のコストを追加
                cost += c[i][0];
                for j in 0..m {
                    und[j] += c[i][j + 1];
                }
                // xを下回る理解度が存在するならば
                // if und.iter().filter(|ga| ga < &&x).count() == 0 {
                if und.iter().all(|ga| ga >= &&x) {
                    // 現在の金額と比較して低いほうに更新
                    min_cost = min(min_cost, cost);
                }
            }
        }
    }
    if min_cost != i32::max_value() {
        println!("{}", min_cost);
    } else {
        println!("-1");
    }
}

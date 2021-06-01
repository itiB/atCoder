use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        k: i32,
    }
    let mut map = HashMap::new();

    // modの世界で見る
    let mut x = 7 % k;
    let mut count = 1;

    // 余りがあるループができたら割り切れないことがわかる
    while !map.contains_key(&x) {
        // どこかであまり0になったら終了
        if x == 0 {
            println!("{}", count);
            return;
        }
        map.insert(x, true);
        x = (10 * x + 7) % k;
        count += 1;
    }
    println!("-1");
}

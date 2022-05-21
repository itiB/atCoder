use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }
    let mut map = HashMap::new();
    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }
    let mut ans = n * (n - 1) * (n - 2) / 6;
    for (_, val) in map {
        if val == 2 {
            ans -= n - 2;
        }
        if val > 2 {
            // 全部dの場合
            ans -= val * (val - 1) * (val - 2) / 6;
            // 2つdの場合
            ans -= val * (val - 1) * (n - val) / 2;
        }
    }
    println!("{}", ans);
}

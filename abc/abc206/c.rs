use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();
    map.entry(a[n - 1]).or_insert(1);
    let mut ans = 0;
    for i in 0..n - 1 {
        // 最後の一つ前から数える
        ans += i + 1;
        if map.contains_key(&a[n - 2 - i]) {
            ans -= map.get(&a[n - 2 - i]).unwrap();
        }
        *map.entry(a[n - 2 - i]).or_insert(0) += 1;
    }
    println!("{}", ans);
}

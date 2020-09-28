use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();
    let mut ans = 0;

    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }

    for (key, val) in map.iter() {
        ans += if val >= key { val - key } else { *val };
    }
    println!("{}", ans);
}
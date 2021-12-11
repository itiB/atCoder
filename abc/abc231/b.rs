use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut map = HashMap::new();
    for a in s {
        *map.entry(a).or_insert(0) += 1;
    }
    let num = &map.values().max().unwrap();
    for (key, value) in &map {
        if value == *num {
            println!("{}", key);
        }
    }
}
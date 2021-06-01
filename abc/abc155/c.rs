use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut map: HashMap<String, usize> = HashMap::new();

    for a in s {
        *map.entry(a).or_insert(0) += 1;
    }

    let num = &map.values().max().unwrap();
    let mut ans = Vec::new();
    for (key, value) in &map {
        if value == *num {
            ans.push(key);
        }
    }
    ans.sort();
    for a in ans {
        println!("{}", a);
    }
}

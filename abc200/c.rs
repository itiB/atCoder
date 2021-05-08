use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut ans = 0;

    for i in a {
        if map.contains_key(&(i % 200)) {
            ans += map.get(&(i % 200)).unwrap();
            *map.entry(i % 200).or_insert(0) += 1;
        } else {
            map.insert(i % 200, 1);
        }
    }
    println!("{}", ans);
}
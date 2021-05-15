use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        st: [(String, usize); n],
    }

    let mut map = BTreeMap::new();
    for (s, t) in st {
        map.insert(t, s);
    }
    let mut iter = map.iter();
    for _ in 0..map.len() - 2 {
        let _ = iter.next();
    }

    println!("{}", iter.next().unwrap().1);
}
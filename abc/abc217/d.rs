use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize, usize); q]
    }

    let mut s = BTreeSet::new();
    s.insert(0);
    s.insert(l);
    for (c, x) in cx {
        match c {
            1 => {
                s.insert(x);
            },
            2 => {
                let r = s.range(x..).next().unwrap();
                let l = s.range(..x).rev().next().unwrap();
                println!("{}", r - l);
            },
            _ => {}
        }
    }
}

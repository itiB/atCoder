use proconio::input;
use std::cmp::{min, max};

fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize,
    }

    let mut mi = 1_000_000_001;
    let mut ma = 0;

    for i in 0..1_000_000 {
        if a * i <= w * 1000 && w * 1000 <= b * i {
            ma = max(ma, i);
            mi = min(mi, i);
        }
    }
    if mi != 1_000_000_001 {
        println!("{} {}", mi, ma);
    } else {
        println!("UNSATISFIABLE");
    }
}

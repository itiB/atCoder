use proconio::input;
use std::cmp::{min, max};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut mi = 1001;
    let mut ma = 0;
    for i in 0..n {
        ma = max(a[i], ma);
        mi = min(b[i], mi);
    }
    println!("{}", if ma > mi { 0 } else { mi - ma + 1 });
}
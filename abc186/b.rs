use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut minimal = std::i32::MAX as usize;
    let mut sum = 0;
    for ha in a {
        for wa in ha {
            sum += wa;
            minimal = min(wa, minimal);
        }
    }
    println!("{}", sum - minimal * h * w);
}
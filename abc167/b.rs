use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        a: i32,
        b: i32,
        _c: i32,
        mut k: i32
    }

    let mut _sum = 0;
    _sum = min(a, k);
    k = k - _sum - b;

    if k > 0 {
        _sum = _sum - k;
    }
    println!("{}", _sum);
}

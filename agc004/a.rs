use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut mid = a / 2;
    let mut ans = ((a - mid) * b * c) - (mid * b * c);
    mid = b / 2;
    ans = min((a * (b - mid) * c) - (a * mid * c), ans);
    mid = c / 2;
    ans = min((a * b * (c - mid)) - (a * b * mid), ans);
    println!("{}", ans);
}
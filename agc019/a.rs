use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        q: usize,
        h: usize,
        s: usize,
        d: usize,
        mut n: usize,
    }

    let mut ans = 0;
    ans += n / 2 * min(min(q * 8, h * 4), min(s * 2, d));
    ans += n % 2 * min(min(q * 4, h * 2), s);
    println!("{}", ans);
}
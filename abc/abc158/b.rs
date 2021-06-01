use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    println!("{}", (n / (a + b)) * a + min(n % (a + b), a));
}
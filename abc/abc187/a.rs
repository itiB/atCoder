use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", max(adder(a), adder(b)));
}

fn adder(n: usize) -> usize {
    let mut sum = 0;

    sum += n / 100;
    sum += n / 10 % 10;
    sum += n % 10;

    return sum;
}
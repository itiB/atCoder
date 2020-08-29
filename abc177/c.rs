use proconio::input;
// use proconio::marker::{Bytes, Chars};

fn main() {
    input! {
        n: usize,
        a: [i128; n],
    }

    let mut last = a[0];
    let mut result = 0;

    for i in 1..n {
        result += a[i] * last;
        result = result % (1000000007);
        last += a[i];
    }

    println!("{}", result);
}

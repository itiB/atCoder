use proconio::input;
// use proconio::marker::{Bytes, Chars};

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    }
    let mut count = 0;
    let dd = d * d;
    for num in xy {
        if num.0 * num.0 + num.1 * num.1 <= dd {
            count += 1;
        }
    }

    println!("{}", count);
}
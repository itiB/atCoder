use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    let ac = a * c;
    let ad = a * d;
    let bc = b * c;
    let bd = b * d;

    println!("{}", max(max(ac, ad), max(bc, bd)));
}
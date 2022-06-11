use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        x: i64,
        a: i64,
        d: i64,
        m: i64
    }
    let n = m - 1;

    if d >= 0 {
        if x <= a {
            println!("{}", a - x);
        } else {
            if x - a <= d * n {
                let tmp = (x - a) % d;
                println!("{}", min(tmp.abs(), d - tmp.abs()));
            } else {
                println!("{}", x - (a + d * n));
            }
        }
    } else {
        if x >= a {
            println!("{}", x - a);
        } else {
            if x - a > d * n {
                let tmp = (x - a) % d;
                println!("{}", min(tmp.abs(), d.abs() - tmp.abs()));
            } else {
                println!("{}", ((a + d * n) - x).abs());
            }
        }
    }
}

use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize
    }

    let taka = (x / (a + c)) * a * b + min(x % (a + c), a) * b;
    let aoki = (x / (d + f)) * d * e + min(x % (d + f), d) * e;
    if taka > aoki {
        println!("Takahashi");
    } else if taka == aoki {
        println!("Draw");
    } else {
        println!("Aoki");
    }
}
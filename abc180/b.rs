use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
    }


    let mut man = 0;
    let mut ma = 0;
    let mut yu = 0;
    for i in &x {
        man += &i.abs();
        ma = max(i.abs(), ma);
        yu += *i * *i;
    }
    println!("{}", man);
    println!("{:.15}", (yu as f64).sqrt());
    println!("{}", ma);
}

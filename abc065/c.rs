use proconio::input;
use std::cmp::{min, max};

fn main() {
    input! {
        m: usize,
        n: usize,
    }

    let div = 1000000007;
    let mut ans = 0;
    match max(m, n) - min(m, n) {
        0 => {
            // 差がないとき
            ans = (power(m, div) * power(m, div)) % div * 2;
        },
        1 => {
            // 差が1の時
            ans = ((power(m, div) * power(n, div)) % div) % div;
        },
        _ => {},
    }
    println!("{}", ans % div);
}

fn power(x: usize, div: usize) -> usize {
    let mut ans = 1;
    for i in 1..=x {
        ans *= i;
        ans %= div;
    }
    ans
}
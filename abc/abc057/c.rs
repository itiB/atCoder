use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
    }

    let mut ans = num_digit(n);

    let mut i = 0;
    loop {
        if i * i > n { break; }
        i += 1;

        if n % i == 0 {
            ans = min(ans, max(num_digit(i), num_digit(n / i)));
        }
    }
    println!("{}", ans);
}

fn num_digit<T: std::string::ToString>(x: T) -> usize {
    let s: Vec<char> = x.to_string().chars().collect();
    s.len()
}

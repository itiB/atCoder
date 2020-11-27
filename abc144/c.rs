use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
    }

    let mut a = 1;
    let mut ans = n;
    while a * a <= n {
        if n % a == 0 {
            let b = n / a;
            ans = min(ans, a + b - 2);
        }
        a += 1;
    }
    println!("{}", ans);
}

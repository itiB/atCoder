use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        w: i128,
        mut ab: [(i128, i128); n]
    }

    ab.sort_by_key(|&x| Reverse(x));
    let mut ans = 0;
    let mut sum = 0;
    for (a, b) in ab {
        sum += b;
        ans += a * b;
        if sum > w {
            let rm = sum - w;
            sum = w;
            ans -= rm * a;
        }
        if sum == w {
            println!("{}", ans);
            return
        }
    }
    println!("{}", ans);
}
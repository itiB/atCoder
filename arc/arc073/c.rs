use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        t: usize,
        time: [usize; n],
    }

    let mut ans = 0;
    for i in 1..n {
        ans += min(t, time[i] - time[i - 1]);
    }
    ans += t;
    println!("{}", ans);
}
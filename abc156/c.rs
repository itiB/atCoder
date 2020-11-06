use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }
    let mut ans = std::i64::MAX as usize;

    for a in 0..100 {
        let mut mid = 0;
        for num in &x {
            if &a > num {
                mid += (a - num) * (a - num);
            } else {
                mid += (num - a) * (num - a);
            }
        }
        ans = min(ans, mid);
    }
    println!("{}", ans);
}
use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut sums = vec![0; n];
    let mut sum = 0;
    let mut far = 0;
    let mut farest = vec![0; n];
    for i in 0..n {
        sum += a[i];
        sums[i] = sum;
        far = max(far, sum);
        farest[i] = far;
    }

    let mut ans = 0;
    sum = 0;
    for i in 0..n {
        ans = max(ans, sum + farest[i]);
        sum += sums[i];
    }
    println!("{}", ans);
}

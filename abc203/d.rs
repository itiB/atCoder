use proconio::input;
use std::cmp::min;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n],
    }

    let mut ans = 1_000_000_001;

    for x in 0..=n-k {
        for y in 0..=n-k {
            let mut v = Vec::new();
            for kx in 0..k {
                for ky in 0..k {
                    v.push(a[kx + x][ky + y]);
                }
            }
            v.sort_by_key(|&x| Reverse(x));
            ans = min(ans, v[k * k / 2])
        }
    }
    println!("{}", ans);
}

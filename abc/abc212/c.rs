use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i32; n],
        mut b: [i32; m]
    }

    a.sort();
    b.sort();
    let mut ans = 1_000_000_001;
    let mut b_min = 0;
    for aa in a {
        for i in b_min..m {
            ans = min(ans, (aa - b[i]).abs());
            if aa >= b[i] {
                b_min = i;
            } else {
                break;
            }
        }
    }
    println!("{}", ans);
}
use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let mut pp = 0;
    let mut qq = 0;
    for (k, r) in (1..n + 1).permutations(n).enumerate() {
        if r == p { pp = k; }
        if r == q { qq = k; }
    }
    println!("{}", if pp > qq { pp - qq } else { qq - pp });
}

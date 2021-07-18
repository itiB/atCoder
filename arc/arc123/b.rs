use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    a.sort_by_key(|&x| Reverse(x));
    b.sort_by_key(|&x| Reverse(x));
    c.sort_by_key(|&x| Reverse(x));

    let mut aa = 0;
    let mut bb = 0;
    let mut cc = 0;

    let mut ans = 0;
    while cc < n {
        if b[bb] < c[cc] {
            println!("{}/ {} {} {}", ans, aa, bb, cc);
            if a[aa] < b[bb] {
                ans += 1;
                aa += 1;
                bb += 1;
                cc += 1;
            } else {
                aa += 1;
            }
            if aa == n { break; }
        } else {
            bb += 1;
            if bb == n { break; }
        }
    }
    println!("{}", ans);
}
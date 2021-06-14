use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i128; n],
    }
    let mut b = VecDeque::from(a);
    b.push_front(0);
    b.push_back(1_000_000_000_000_000_001);

    for _ in 0..q {
        input! { mut k: i128 }
        let mut l = 0;
        let mut r = 1_000_000_000_010_000_001;

        while r - 1 > l {
            // m 以下にk個値が存在するか
            let m = (l + r) / 2;
            if m - get_min_bound(m as i128, &b) < k {
                l = m;
            } else {
                r = m;
            }
        }
        println!("{}", r);
    }
}

fn get_min_bound(k: i128, b: &VecDeque<i128>) -> i128 {
    let mut l = 0;
    let mut r = b.len() - 1;

    while r - 1 > l {
        // kを超えない最大の値を調べる
        let m = (l + r) / 2;
        if b[m] > k {
            r = m;
        } else {
            l = m;
        }
    }
    return l as i128
}
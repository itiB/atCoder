use proconio::input;
use std::collections::HashMap;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n]
    }

    let mut s: HashMap<usize, usize> = HashMap::new();
    for i in n - k..n {
        *s.entry(c[i]).or_insert(0) += 1;
    }
    let mut ans = s.len();
    for i in (0..n - k).rev() {
        *s.entry(c[i]).or_insert(0) += 1;
        *s.entry(c[i + k]).or_insert(0) -= 1;
        if s.get(&c[i + k]).unwrap() == &0 {
            s.remove(&c[i + k]);
        }
        ans = max(ans, s.len());
    }
    println!("{}", ans);
}
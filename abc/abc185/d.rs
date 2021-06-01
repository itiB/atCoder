use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }

    if n == m {
        println!("0");
    } else if m == 0 {
        println!("1");
    } else {
        let mut l = 0;
        let mut len = 1_000_000_001;
        a.sort();
        // 最小の幅を求める
        for i in &a {
            if i - l > 1 {
                len = min(len, i - l - 1);
            }
            l = *i;
        }
        l = 0;
        let mut ans = 0;
        for i in &a {
            if i - l > 1 {
                ans += (i - l - 1 + len - 1) / len;
            }
            l = *i;
        }
        if l != n {
            ans += (n - l + len - 1) / len;
        }
        println!("{}", ans);
    }
}
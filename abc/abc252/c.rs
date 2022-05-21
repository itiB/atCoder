use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut m = vec![HashSet::new(); 10];
    for ss in s {
        for i in 0..10 {
            let mut tmp = i;
            for _ in 0..=n {
                if m[ss[i].to_digit(10).unwrap() as usize].contains(&tmp) {
                    tmp += 10;
                } else {
                    m[ss[i].to_digit(10).unwrap() as usize].insert(tmp);
                    break;
                }
            }
        }
    }
    let mut ans = n * 20;
    for mm in m {
        let mut sum = 0;
        for i in mm {
            sum = max(sum, i);
        }
        ans = min(ans, sum);
    }
    println!("{}", ans);
}

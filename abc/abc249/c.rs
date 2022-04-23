use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n]
    }

    let mut ans = 0;
    for i in 1..1usize << n {
        let mut count = [0; 26];
        for j in 0..n {
            if i >> j & 1 == 1 {
                for c in &s[j] {
                    count[(*c as i32 - 'a' as i32) as usize] += 1;
                }
            }
        }
        let mut sum = 0;
        for l in &count {
            if *l == k {
                sum += 1;
            }
        }
        ans = max(ans, sum);
    }
    println!("{}", ans);
}
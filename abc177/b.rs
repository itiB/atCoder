use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut change = 10000;

    for i in 0..=s.len() - t.len() {
        let mut matched = 0;
        for j in 0..t.len() {
            if t[j] == s[j + i] {
                matched += 1;
            }
        }
        change = min(t.len() - matched, change);
    }

    println!("{}", change);
}
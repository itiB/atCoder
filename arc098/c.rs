use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut map = Vec::new();

    let mut num = 0;
    for c in &s {
        match c {
            'E' => num += 1,
            _ => {},
        }
        map.push(num);
    }

    let mut ans = 1_000_000;
    for i in 0..s.len() {
        let mut t = 0;
        // リーダより右側でWを向いていない者たちの数
        t += if i < s.len() - 2 { map[s.len() - 1] - map[i] } else { 0 };
        // リーダより左側でEを向いていない者たちの数
        t += if i > 1 { i - map[i - 1] } else { 0 };

        ans = min(ans, t);
    }

    println!("{}", ans);
}
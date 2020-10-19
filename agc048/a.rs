use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        input! {
            s: String,
        }
        if "atcoder" < &s {
            println!("0");
        } else {
            let mut map = Vec::new();
            let st = s.chars().collect::<Vec<char>>();
            for c in &st {
                if c > &'a' {
                    map.push(c);
                }
            }
            if map.len() > 0 {
                let mut ans = 1_000_000_000;
                for i in 1..s.len() {
                    if st[i] > 'a' {
                        ans = min(ans, i);
                        break;
                    }
                }
                for i in 2..s.len() {
                    if st[i] > 't' {
                        ans = min(ans, i - 1);
                        break;
                    }
                }
                println!("{}", ans);
            } else {
                println!("-1");
            }
        }
    }
}

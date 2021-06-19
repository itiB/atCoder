use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
        q: usize
    }
    let mut ans: VecDeque<char> = VecDeque::from(s);

    let mut rev = false;
    for _ in 0..q {
        input! {t: usize}
        if t == 2 {
            input! {
                f: usize,
                c: char,
            }
            if rev && f == 2 || !rev && f == 1 {
                ans.push_front(c);
            } else {
                ans.push_back(c);
            }
        } else {
            rev ^= true;
        }
    }

    if !rev {
        println!("{}", ans.iter().collect::<String>());
    } else {
        println!("{}", ans.iter().rev().collect::<String>());
    }
}
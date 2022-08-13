use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars
    }
    let mut memo = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((s, 0));
    while let Some((mut v, count)) = q.pop_front() {
        if v[0] == 'a'
            && v[1] == 't'
            && v[2] == 'c'
            && v[3] == 'o'
            && v[4] == 'd'
            && v[5] == 'e'
            && v[6] == 'r'
        {
            println!("{}", count);
            return;
        }
        for i in 0..=5 {
            // i, i+1を入れ替える
            v.swap(i, i + 1);
            if !memo.contains(&v) {
                memo.insert(v.to_vec());
                q.push_back((v.to_vec(), count + 1));
            }
            v.swap(i, i + 1);
        }
    }
}

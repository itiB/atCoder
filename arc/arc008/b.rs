use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        _: usize,
        _: usize,
        name: Chars,
        kit: Chars,
    }
    let mut kits = HashMap::new();
    let mut names = HashMap::new();

    for k in &kit {
        *kits.entry(*k).or_insert(0) += 1;
    }
    for n in &name {
        *names.entry(*n).or_insert(0) += 1;
    }
    let mut ans: i32 = 1;

    for c in &name {
        if kits.contains_key(c) {
            *kits.entry(*c).or_insert(0) -= 1;
            if kits.get(c).unwrap() < &0 {
                ans += 1;
                for k in &kit {
                    *kits.entry(*k).or_insert(0) += 1;
                }
            }
        } else {
            ans = -1;
            break
        }
    }
    println!("{}", ans);
}
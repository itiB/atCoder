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
    let a_z = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut kits = a_z.chars().map(|c| (c, 0)).collect::<HashMap<_, i32>>();
    let mut back = a_z.chars().map(|c| (c, 0)).collect::<HashMap<_, i32>>();
    let mut names = a_z.chars().map(|c| (c, 0)).collect::<HashMap<_, i32>>();

    for k in &kit {
        *kits.entry(*k).or_insert(0) += 1;
        *back.entry(*k).or_insert(0) += 1;
    }
    for n in &name {
        *names.entry(*n).or_insert(0) += 1;
    }
    let mut ans: i32 = 1;

    for c in &name {
        if back.get(c).unwrap() == &0 {
            ans = -1;
            break
        }
        *kits.entry(*c).or_insert(0) -= 1;
        if kits.get(c).unwrap() < &0 {
            ans += 1;
            for k in &kit {
                *kits.entry(*k).or_insert(0) += 1;
            }
        }
    }
    println!("{}", ans);
}
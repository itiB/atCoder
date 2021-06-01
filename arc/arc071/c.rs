use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let a_z = "abcdefghijklmnopqrstuvwxyz";
    let mut counts = vec![a_z.chars().map(|c| (c, 0)).collect::<HashMap<_, _>>(); n];

    for i in 0..s.len() {
        for c in &s[i] {
            *counts[i].entry(*c).or_insert(0) += 1;
        }
    }

    for c in a_z.chars() {
        let mut counter = 51;
        for map in &counts {
            counter = min(counter, *map.get(&c).unwrap());
        }
        for _ in 0..counter {
            print!("{}", c);
        }
    }
    println!("");
}

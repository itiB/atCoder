use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; k]
    }
    let set: HashSet<usize> = b.into_iter().collect();
    let max = &a.iter().max().unwrap();
    let mut flag = false;
    for i in 0..a.len() {
        if a[i] == **max && set.contains(&(i + 1)) {
            flag = true;
        }
    }
    println!("{}", if flag { "Yes" } else { "No" });
}

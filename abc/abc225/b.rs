use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1]
    }

    let mut branch = vec![HashSet::new(); 1+n];
    for (a, b) in ab {
        branch[a].insert(b);
        branch[b].insert(a);
    }
    for i in 0..=n {
        if branch[i].len() == n-1 {
            println!("Yes");
            return
        }
    }
    println!("No");
}
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", search(0, n));
}

fn search(n: usize, lim: usize) -> usize {
    if n > lim {
        return 0
    }
    if checker(n) {
        search(n * 10 + 3, lim) + search(n * 10 + 5, lim) + search(n * 10 + 7, lim) + 1
    } else {
        search(n * 10 + 3, lim) + search(n * 10 + 5, lim) + search(n * 10 + 7, lim)
    }
}

fn checker(n: usize) -> bool {
    let num: HashSet<char> = n.to_string().chars().collect();
    if num.contains(&'3') && num.contains(&'5') && num.contains(&'7') {
        true
    } else {
        false
    }
}

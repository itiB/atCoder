use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
    }
    let MAX = 100000;

    let mut list: Vec<usize> = vec![1];
    let mut i = 6;
    while i <= MAX {
        list.push(i);
        i *= 6;
    }
    i = 9;
    while i <= MAX {
        list.push(i);
        i *= 9;
    }
    list.sort();
    let mut dp: Vec<Option<usize>> = vec![None; n + 1];
    dp[0] = Some(1);
    for l in &list {
        if *l <= n {
            dp[*l] = Some(1);
        }
    }
    for i in 1..=n {
        match dp[i] {
            Option::None => {
                let mut v = MAX;
                for l in &list {
                    if i >= *l {
                        v = min(v, dp[i - l].unwrap() + 1);
                    }
                }
                dp[i] = Some(v);
            },
            Option::Some(_) => continue,
        }
    }
    println!("{}", dp[n].unwrap());
}
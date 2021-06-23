use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        mut l: usize,
        mut r: usize,
    }

    let mut ans = l * r % 2019;
    for i in l..r {
        for j in i + 1..=r {
            ans = min(ans, i * j % 2019);
            if ans == 0 { break }
        }
        if ans == 0 { break }
    }
    println!("{}", ans);
}
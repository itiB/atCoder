use proconio::input;
use proconio::marker::Chars;

fn solve() -> bool {
    input! {
        h: usize,
        _: usize,
        a: [Chars; h],
    }

    let mut left = 0;
    let mut right = 0;
    for line in a {
        for i in 0..left {
            if line[i] == '#' {
                return false
            }
        }
        for i in left..line.len() {
            if line[i] == '#' {
                left = i;
                right = i;
            } else {
                right = i;
                break
            }
        }
        for i in right + 1..line.len() {
            if line[i] == '#' {
                return false
            }
        }
    }
    true
}

fn main() {
    if solve() {
        println!("Possible");
    } else {
        println!("Impossible");
    }
}
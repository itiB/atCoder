use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q],
    }

    let mut map = vec![0; n];
    let mut count = 0;
    for i in 1..s.len() {
        if s[i - 1] == 'A' && s[i] == 'C' {
            count += 1;
        }
        map[i] = count;
    }
    for (l, r) in lr {
        println!("{}", map[r - 1] - map[l - 1]);
    }
}
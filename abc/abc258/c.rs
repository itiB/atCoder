use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars
    }
    let mut start = 0;
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        if t == 2 {
            println!("{}", s[(start + x - 1) % n]);
        } else {
            start += n - x;
        }
    }
}

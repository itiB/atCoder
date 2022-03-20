use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        t: Chars
    }

    let mut x = 0;
    let mut y = 0;
    let mut a = 0;
    let w = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
    for c in t {
        if c == 'R' {
            a += 1;
        } else {
            x += w[a % 4].0;
            y += w[a % 4].1;
        }
    }
    println!("{} {}", x, y);
}
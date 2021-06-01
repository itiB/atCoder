use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut counter = 0;
    let mut max = 0;

    for c in s {
        if c == 'R' {
            counter += 1;
            if max < counter {
                max = counter;
            }
        } else {
            counter = 0
        }
    }
    println!("{}", max);
}
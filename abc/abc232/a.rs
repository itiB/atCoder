use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    println!("{}", c2u(s[0]) * c2u(s[2]));
}

fn c2u(c: char) -> usize {
    c as usize - '0' as usize
}
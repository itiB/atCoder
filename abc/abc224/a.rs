use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    if s[s.len() - 1] == 'r' {
        println!("er");
    } else {
        println!("ist");
    }
}
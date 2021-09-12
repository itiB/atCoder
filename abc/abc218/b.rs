use proconio::input;
use std::char;
fn main() {
    input! {
        p: [u8; 26]
    }

    for c in p {
        print!("{}", ('a' as u8 + c - 1) as char);
    }
}
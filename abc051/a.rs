use proconio::input;
// use proconio::marker::{Bytes, Chars};

fn main() {
    input! {
        s: String,
    }

    println!("{}", s.replace(",", " "));
    
}
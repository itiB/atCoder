use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        k: usize,
        s: Chars,
    }

    if s.len() <= k {
        for i in s {
            print!("{}", i);
        }
        println!("");
    } else {
        for i in 0..k {
            print!("{}", s[i]);
        }
        println!("...");
    }
}
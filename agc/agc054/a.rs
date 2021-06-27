use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    if s[0] != s[n - 1] {
        println!("1");
    } else {
        let mut f = false;
        for i in 1..n {
            if s[i - 1] != s[0] && s[i] != s[0] { f = true; break }
        }
        println!("{}", if f { 2 } else { -1 });
    }
}
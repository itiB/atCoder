use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    for i in 0..n {
        if s[i] == '0' {
            continue;
        }
        println!("{}", if i % 2 == 0 {
            "Takahashi"
        } else {
            "Aoki"
        });
        break;
    }
}
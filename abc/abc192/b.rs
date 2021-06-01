use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut ans: bool = true;
    for i in 0..s.len() {
        if i % 2 == 0 {
            if 'A' <= s[i] && s[i] <= 'Z' {
                ans = false;
            }
        } else {
            if 'a' <= s[i] && s[i] <= 'z' {
                ans = false;
            }
        }
    }
    if ans == true {
        println!("Yes");
    } else {
        println!("No");
    }
}
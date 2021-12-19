use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut k = c2u(t[0]) - c2u(s[0]);
    if k < 0 { k += 26 }
    for i in 1..s.len() {
        let mut tmp = c2u(t[i]) - c2u(s[i]);
        if tmp < 0 { tmp += 26 }
        if tmp != k {
            println!("No");
            return
        }
    }
    println!("Yes");
}

fn c2u(c: char) -> i32 {
    c as i32 - '0' as i32
}
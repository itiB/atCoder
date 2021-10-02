use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut c = Vec::new();
    for i in 0..s.len() {
        if s[i] != t[i] { c.push(i) }
    }
    if c.len() == 0 {
        println!("Yes");
        return
    }
    if c.len() == 1 {
        println!("No");
        return
    }
    if c.len() > 3 {
        println!("No");
        return
    }
    if c[1] - c[0] != 1 {
        println!("No");
        return
    }
    if t[c[0]] == s[c[1]] && t[c[1]] == s[c[0]] {
        println!("Yes");
    } else {
        println!("No");
    }
}
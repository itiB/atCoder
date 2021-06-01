use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
        t: Chars,
    }

    if n.len() + 1 != t.len() {
        println!("No");
        return;
    }
    for i in 0..n.len() {
        if n[i] != t[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
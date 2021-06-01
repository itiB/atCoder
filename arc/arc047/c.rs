use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut map = Vec::new();
    map.push(s[0]);
    for i in 1..s.len() {
        if map[map.len() - 1] != s[i] {
            map.push(s[i]);
        }
    }
    println!("{}", map.len() - 1);
}
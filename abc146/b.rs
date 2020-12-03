use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: u8,
        s: Chars,
    }
    let ans: String = s.iter().map(|c| (((*c as u8 - 'A' as u8 + n) % 26 + 'A' as u8) as char)).collect();
    println!("{}", ans);
}
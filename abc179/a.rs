use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let len = s.len();
    if s[len - 1] == 's' {
        for c in &s {
            print!("{}", c);
        }
        println!("es");
    } else {
        for c in &s {
            print!("{}", c);
        }
        println!("s");
    }
}
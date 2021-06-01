use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    }

    let mut sum: i64 = 9;

    for c in n {
        sum += c as i64 - 48;
    }

    if sum % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
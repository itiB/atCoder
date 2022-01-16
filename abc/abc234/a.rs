use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: Chars
    }

    println!("{}", x(a[0], a[1], a[2])+x(a[1], a[2], a[0]) + x(a[2], a[0], a[1]));
}

fn x(a: char, b: char, c: char) -> usize {
    let mut ans: u32 = 0;
    ans += a.to_digit(10).unwrap() * 100;
    ans += b.to_digit(10).unwrap() * 10;
    ans += c.to_digit(10).unwrap();
    return ans as usize
}
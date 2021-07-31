use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
    }

    if x[0] == x [1] && x[0] == x[2] && x[0] == x[3] {
        println!("Weak");
        return
    }
    if (x[3].to_digit(10).unwrap() as i32 - x[2].to_digit(10).unwrap() as i32 + 10) % 10 == 1
        && (x[2].to_digit(10).unwrap() as i32 - x[1].to_digit(10).unwrap() as i32 + 10) % 10 == 1
        && (x[1].to_digit(10).unwrap() as i32 - x[0].to_digit(10).unwrap() as i32 + 10) % 10== 1 {
            println!("Weak");
            return
        }
    println!("Strong");
}
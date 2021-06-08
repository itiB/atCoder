use proconio::input;
use std::char;
use std::collections::VecDeque;

fn main() {
    input! {
        mut n: i64,
    }

    let mut ans = VecDeque::new();

    while n > 0 {
        if n % 26 == 0 {
            ans.push_front('z');
            n = n / 26 - 1;
        } else {
            ans.push_front(char::from_digit((9 + n % 26) as u32, 36).unwrap());
            n /= 26;
        }
    }

    for a in ans {
        print!("{}", a);
    }
    println!();
}
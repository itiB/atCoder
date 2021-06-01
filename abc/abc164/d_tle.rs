use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }
    let mut numbers = Vec::new();
    let mut _num = 0;
    let mut _counter = 0;
    let mut result = 0;

    for _num in s {
        numbers.push(_num as i64 - 48);
    }

    for i in 0..numbers.len() {
        for j in i + 4..numbers.len() {
            _num = 0;
            _counter = 0;
            for k in (i..=j).rev() {
                _num += numbers[k] * 10_i64.pow(_counter);
                _counter += 1;
            }
            if _num % 2019 == 0 {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        n: i64,
    }
    let mut _max = 0;

    if n >= b - 1 {
        // b - 1で最大
        _max = a * (b - 1) / b - a * ((b - 1) / b);
    } else {
        // nで最大になる
        _max = a * n / b - a * (n / b);
    }
    println!("{}", _max);
}
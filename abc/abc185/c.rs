use proconio::input;

fn main() {
    input! {
        l: i64,
    }

    println!("{}", combination(l - 1, 11));
}

fn combination(mut n: i64, k: i64) -> i64 {
    let mut result = 1;
    for i in 1..=k {
        result = result * n / i;
        n = n - 1;
    }
    result
}
use proconio::input;

fn main() {
    input! {
        mut n: i128,
    }

    let mut sum = n / 11 * 2;
    n = n % 11;
    if n > 6 { sum += 2 }
    if 6 >= n &&n > 0 { sum += 1 }

    println!("{}", sum);
}
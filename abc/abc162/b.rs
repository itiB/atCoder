use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut sum: i64 = 0;

    for i in 1..n + 1 {
        if !((i % 3 == 0) || (i % 5 == 0)) {
            sum = sum + i;
        }
    }
    println!("{}", sum);
}
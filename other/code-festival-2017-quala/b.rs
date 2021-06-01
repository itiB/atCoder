use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
        k: i64,
    }

    for a in 0..=n {
        for b in 0..=m {
            if a * m + b * n - 2 * a * b == k {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
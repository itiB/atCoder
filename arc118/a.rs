use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
    }

    println!("{}", (n * 100 + t - 1) / t + n - 1);
}
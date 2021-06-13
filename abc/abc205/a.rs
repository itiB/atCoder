use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", (a * b) as f64 / 100.0);
}
use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    println!("{}", 100.0 - 100.0 * b / a);
}
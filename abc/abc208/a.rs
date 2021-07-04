use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    println!("{}", if a <= b && b <= 6 * a { "Yes" } else { "No" });
}
use proconio::input;

fn main() {
    input! {
        h1: i32,
        m1: i32,
        h2: i32,
        m2: i32,
        k: i32
    }

    println!("{}", 60 * (h2 - h1) + m2 - m1 - k);
}
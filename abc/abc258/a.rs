use proconio::input;

fn main() {
    input! {
        k: usize,
    }
    println!("{}:{1: >02}", 21 + k / 60, k % 60);
}

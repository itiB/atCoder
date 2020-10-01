use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }

    for num in a..=b {
        if num - a < k || b - num < k {
            println!("{}", num);
        }
    }
}
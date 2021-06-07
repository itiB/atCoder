use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    println!("{}", if n / 2 >= k { "YES" } else { "NO" });
}
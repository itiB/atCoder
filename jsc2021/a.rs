use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize
    }

    println!("{}", (y * z - 1) / x);
}
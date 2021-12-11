use proconio::input;

fn main() {
    input! {
        d: usize,
    }

    println!("{}", d as f32 / 100.0);
}
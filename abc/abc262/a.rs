use proconio::input;

fn main() {
    input! {
        y: usize,
    }
    println!(
        "{}",
        y + match y % 4 {
            0 => 2,
            1 => 1,
            2 => 0,
            3 => 3,
            _ => 0,
        }
    );
}

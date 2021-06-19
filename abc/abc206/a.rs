use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if 108 * n / 100 == 206 {
        println!("so-so");
    } else if 108 * n / 100 < 206 {
        println!("Yay!");
    } else {
        println!(":(");
    }
}
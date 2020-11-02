use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if n % 2 == 1 {
        println!("Black");
    } else {
        println!("White");
    }
}
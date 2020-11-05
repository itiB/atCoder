use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if (10 * a) / 100 == (8 * b) / 100 {
        println!("{}", a * 100 / 8);
    } else {
        println!("-1");
    }
}
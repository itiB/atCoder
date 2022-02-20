use proconio::input;

fn main() {
    input! {
        h: usize
    }

    println!("{}", (((12800000 + h) * h) as f64).sqrt());
}
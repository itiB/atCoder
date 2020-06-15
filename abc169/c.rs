use proconio::input;

fn main() {
    input! {
        a: u64,
        b: f64,
    }
    // 1.19 -> 118.999 ... + 0.5 -> 119.222
    // 1.18 -> 118.000
    let bb = (b * 100.0 + 0.5).floor() as u64;
    println!("{}", (a * bb) / 100);
}
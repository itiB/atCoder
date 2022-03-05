use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize
    }

    let mut ans = 0.0;
    if x <= a {
        ans = 1.0;
    } else if x <= b {
        ans = c as f64 / (b as f64 - a as f64);
    }
    println!("{}", ans);
}
use proconio::input;

fn main() {
    input! {
        v: usize,
        t: usize,
        s: usize,
        d: usize,
    }

    if  v * t > d || v * s < d {
        println!("Yes");
    } else {
        println!("No");
    }
}
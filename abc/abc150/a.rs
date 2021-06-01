use proconio::input;

fn main() {
    input! {
        k: usize,
        x: usize,
    }

    if k * 500 >= x {
        println!("Yes");
    } else {
        println!("No");
    }
}
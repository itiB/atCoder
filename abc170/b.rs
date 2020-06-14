use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }
    for n in 0..=x {
        if y == 2 * n + (x - n) * 4 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
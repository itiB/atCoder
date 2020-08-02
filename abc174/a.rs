use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    if n >= 30 {
        println!("Yes");
    } else {
        println!("No");
    }
}
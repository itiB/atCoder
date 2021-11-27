use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }

    loop {
        if a % 10 + b % 10 >= 10 {
            println!("Hard");
            return
        }
        if a < 10 || b < 10 {
            break
        }
        a /= 10;
        b /= 10;
    }
    println!("Easy");
}
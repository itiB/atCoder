use proconio::input;

fn main() {
    input! {
        l: i32,
        r: i32,
        d: i32,
    }

    if l % d == 0 {
        println!("{}", r / d - l / d + 1);
    } else {
        println!("{}", r / d - l / d);
    }
}
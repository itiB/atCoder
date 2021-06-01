use proconio::input;

fn main() {
    input! {
        l: i32,
        r: i32,
        d: i32,
    }

    println!("{}", r / d - (l - 1) / d);
}
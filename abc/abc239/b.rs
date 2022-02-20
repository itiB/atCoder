use proconio::input;

fn main() {
    input! {
        x: i64
    }

    let t = x % 10;
    let u = if x < 0 { -1 } else { 0 };

    println!("{}", x / 10 + if t != 0 {u} else {0});
}
use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    if x < 90 {
        println!("{}", if x < 40 { 40 - x } else if x < 70 { 70 - x} else { 90 - x });
    } else {
        println!("expert");
    }
}
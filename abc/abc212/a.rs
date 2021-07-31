use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    if a > 0 {
        if b > 0 {
            println!("Alloy");
        } else {
            println!("Gold");
        }
    } else {
        println!("Silver")
    }
}
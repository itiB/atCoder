use proconio::input;

fn main() {
    input! {
        k: i32,
        a: i32,
        b: i32,
    }

    if a / k < b / k {
        println!("OK");
    } else {
        if a % k == 0 || b % k == 0 {
            println!("OK");
        } else {
            println!("NG");
        }
    }
}
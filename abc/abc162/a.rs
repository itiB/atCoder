use proconio::input;

fn main() {
    input! {
        n: i32
    }

    let n_100 = n / 100;
    let n_10 = n / 10 % 10;
    let n_1 = n % 10;

    if n_100 == 7 || n_10 == 7 || n_1 == 7 {
        println!("Yes");
    } else {
        println!("No");
    }
}
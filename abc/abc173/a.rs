use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let mut sum: i32 = 1000;

    loop {
        if sum >= n {
            println!("{}", sum - n);
            break;
        }
        sum += 1000;
    }
}
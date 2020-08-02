use proconio::input;

fn main() {
    input! {
        k: i128,
    }
    let max: i128 = 1_000_000;
    let mut seven: i128 = 7;
    let mut count = 1;
    while count < max {
        if seven % k == 0 {
            println!("{}", count);
            return;
        }
        seven += 7 * 10_i128.pow(count as u32) as i128;
        println!("{} {}", seven, count);
        count += 1;
    }
    println!("-1");
}


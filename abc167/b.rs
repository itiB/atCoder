use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        _c: i32,
        mut k: i32
    }

    let mut sum = 0;

    if a <= k {
        sum += a;
    } else {
        sum = k;
    }
    k = k - a;

    k = k - b;

    if k > 0 {
        sum = sum - k;
    }
    println!("{}", sum);
}

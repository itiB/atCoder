use proconio::input;

fn main() {
    input! {
        x: i32,
        n: i32,
        p: [i32; n],
    }
    for a in 0..=x {
        let num1 = x - a;
        if !p.iter().any(|s| s == &num1) {
            println!("{}", num1);
            return;
        }
        let num2 = x + a;
        if !p.iter().any(|s| s == &num2) {
            println!("{}", num2);
            return;
        }
    }
    // 2x以降
    for a in (2 * x)..=100 {
        if !p.iter().any(|s| s == &a) {
            println!("{}", a);
            return;
        }
    }
}
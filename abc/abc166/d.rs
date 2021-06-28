use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    for a in 0..200 {
        let a5 = a * a * a * a * a;
        for b in -200..200 {
            let b5 = b * b * b * b * b;
            if a5 - b5 == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}
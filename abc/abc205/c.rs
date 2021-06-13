use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64
    }

    if c % 2 == 0 {
        println!("{}",if a.abs() == b.abs() {
                "="
            } else if a.abs() > b.abs() {
                ">"
            } else {
                "<"
            });
    } else {
        println!("{}",if a == b {
                "="
            } else if a > b {
                ">"
            } else {
                "<"
            });
    }
}
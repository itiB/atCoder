use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        a11: usize,
        a12: usize,
        a21: usize,
        a22: usize
    }

    println!(
        "{}",
        if r == 1 {
            if c == 1 {
                a11
            } else {
                a12
            }
        } else {
            if c == 1 {
                a21
            } else {
                a22
            }
        }
    );
}

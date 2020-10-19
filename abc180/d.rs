use proconio::input;
fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize
    }

    let mut power = x;
    let mut ex = 0;

    if b > a && power <= b {
        while power * a <= power + b && power * a < y {
            power *= a;
            ex += 1;
        }
    }
    if y > power {
        ex += (y - power - 1) / b;
    }
    println!("{}", ex);
}

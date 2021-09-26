use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    }

    let i = a / c;
    let j = b / c;
    for x in i..=j {
        if a <= c * x && c * x <= b {
            println!("{}", c*x);
            return
        }
    }
    println!("-1");
}
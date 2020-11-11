use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut sum = 0;
    for (a, b) in ab.iter().rev() {
        let now = sum + a;
        if now % b != 0 {
            sum += b - (now % b);
        }
    }
    println!("{}", sum);
}
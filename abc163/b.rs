use proconio::input;

fn main() {
    input! {
        n: i64,
        m: usize,
        a: [i64; m], // Vec::<i64>
    }

    if n >= a.iter().sum() {
        println!("{}", n - a.iter().sum::<i64>());
    } else {
        println!("-1");
    }
}

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    for i in 0..10001 {
        if i * 8 / 100 == a && i * 10 / 100 == b {
            println!("{}", i);
            return
        }
    }
    println!("-1");
}
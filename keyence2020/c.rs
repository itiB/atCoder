use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: usize,
    }

    for _ in 0..k {
        print!("{} ", s);
    }
    for _ in 0..(n - k) {
        if s == 1_000_000_000 {
            print!("{} ", 1);
        } else {
            print!("{} ", s + 1);
        }
    }
    println!();
}

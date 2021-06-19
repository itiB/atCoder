use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut s = 0;
    let mut d = 1;
    loop {
        s += d;
        d += 1;
        if s >= n {
            println!("{}", d - 1);
            return;
        }
    }
}
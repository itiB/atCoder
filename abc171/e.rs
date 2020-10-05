use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut all = 0;
    for x in &a {
        all ^= x;
    }
    for x in &a {
        print!("{} ", all ^ x);
    }
}
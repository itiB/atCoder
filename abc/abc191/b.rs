use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }

    for c in a {
        if c != x {
            print!("{} ", c);
        }
    }
    println!("");
}
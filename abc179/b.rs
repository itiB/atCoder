use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [(usize, usize); n],
    }

    let mut q = 0;
    for (a, b) in d {
        if a == b {
            q += 1;
        } else {
            q = 0;
        }
        if q == 3 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
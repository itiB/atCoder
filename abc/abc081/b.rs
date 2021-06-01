use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut minimal = 1_000_000_001;
    for num in a {
        let mut i = num;
        let mut count = 0;
        while i % 2 == 0 {
            i /= 2;
            count += 1;
        }
        minimal = min(minimal, count);
    }
    println!("{}", minimal);
}

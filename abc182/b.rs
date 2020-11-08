use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut tar = 2;
    for i in 2..=1000 {
        let mut count = 0;
        for num in &a {
            if num % i == 0 {
                count += 1;
            }
        }
        if ans < count {
            ans = count;
            tar = i;
        }
    }
    println!("{}", tar);
}


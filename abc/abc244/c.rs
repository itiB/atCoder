use proconio::input;
use std::io::{self, Write};

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![false; 2*n+2];
    ans[0] = true;

    for _ in 0..=n {
        for i in 1..=2*n+1 {
            if !ans[i] {
                println!("{}", i);
                ans[i] = true;
                io::stdout().flush().unwrap();
                break
            }
        }
        input! {
            aoki: usize
        }
        ans[aoki] = true;
    }
}
use proconio::input;
use itertools::Itertools;
use std::cmp::{min, max};

fn main() {
    input! {
        n: usize
    }

    // let mut map = vec![vec![0; 2*n-1]; 2*n-1];
    // for i in 0..2*n {
    //     input! { m: [usize; 2*n-i-1]}
    //     for j in i..2*n-1 {
    //         map[i][j] = m[j-i];
    //     }
    // }
    // let mut ans = 0;
    for elt in (0..2*n).permutations(2*n) {
        println!("{:?}", elt);
        // for i in 0..n {
        //     let mini = min(elt[2*i], elt[2*i+1]);
        //     let maxi = max(elt[2*i], elt[2*i+1]);
        //     ans ^= map[mini-1][maxi-1];
        // }
    }
    // println!("{}", ans);
}


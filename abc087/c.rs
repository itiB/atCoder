use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a1: [i32; n],
        a2: [i32; n],
    }

    let mut map = vec![vec![std::i32::MAX; n]; 2];

    map[0][0] = a1[0];
    map[1][0] = a1[0] + a2[0];
    for j in 1..n {
        map[0][j] = map[0][j - 1] + a1[j];
        map[1][j] = max(map[0][j] + a2[j], map[1][j - 1] + a2[j]);
    }
    println!("{}", map[1][n - 1]);
}
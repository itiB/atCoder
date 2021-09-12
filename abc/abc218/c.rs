use proconio::input;
use proconio::marker::Chars;
use std::cmp::{min, max};

fn main() {
    input! {
        n: usize,
        s: [Chars;n],
        t: [Chars;n]
    }

    let mut s_90 = vec![vec![' '; n];n];
    let mut s_180 = vec![vec![' '; n];n];
    let mut s_270 = vec![vec![' '; n];n];
    let mut x_min = 201;
    let mut x_max = 0;
    let mut y_min = 201;
    let mut y_max = 0;
    for x in 0..n {
        for y in 0..n {
            if s[x][y] == '#' {
                x_min = min(x_min, x);
                y_min = min(y_min, y);
                x_max = max(x_max, x);
                y_max = max(y_max, y);
            }
            s_90[y][n-1-x] = s[x][y];
            s_180[n-1-x][n-1-y] = s[x][y];
            s_270[n-1-y][x] = s[x][y];
        }
    }
    let mut tx_min = 201;
    let mut ty_min = 201;
    for x in 0..n {
        for y in 0..n {
            if t[x][y] == '#' {
                tx_min = min(tx_min, x);
                ty_min = min(ty_min, y);
            }
        }
    }
    println!();
    for c in s_90 {
        println!("{:?}", c);
    }
    println!();
    for c in s_180 {
        println!("{:?}", c);
    }
    println!();
    for c in s_270 {
        println!("{:?}", c);
    }
}
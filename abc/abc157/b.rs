use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n],
    }

    let mut open = [[0; 3]; 3];
    let map: HashSet<usize> = b.into_iter().collect();
    for x in 0..3 {
        for y in 0..3 {
            if map.contains(&a[x][y]) {
                open[x][y] = 1;
            }
        }
    }

    for z in 0..3 {
        if open[z][0] * open[z][1] * open[z][2] == 1 {
            println!("Yes");
            return
        }
        if open[0][z] * open[1][z] * open[2][z] == 1 {
            println!("Yes");
            return
        }
    }
    if open[0][0] * open[1][1] * open[2][2] == 1 {
        println!("Yes");
        return
    }
    if open[0][2] * open[1][1] * open[2][0] == 1 {
        println!("Yes");
        return
    }
    println!("No");
}
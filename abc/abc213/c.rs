use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        _: usize,
        _: usize,
        n: usize,
        ab: [(usize, usize); n]
    }

    let mut a = BTreeSet::new();
    let mut b = BTreeSet::new();
    for (i, j) in &ab {
        a.insert(*i);
        b.insert(*j);
    }
    let aa: Vec<usize> = a.iter().cloned().collect();
    let bb: Vec<usize> = b.iter().cloned().collect();

    for (i, j) in &ab {
        println!("{} {}", binary_search(&aa, *i)+1, binary_search(&bb, *j)+1);
    }
}

fn binary_search(array: &Vec<usize>, v: usize) -> usize {
    let mut left = -1_i64;
    let mut right = array.len() as i64;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if array[mid as usize] >= v {
            right = mid;
        } else {
            left = mid;
        }
    }
    right as usize
}


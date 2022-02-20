use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }

    let mut list = vec![true; 250];
    list[0] = false;
    list[1] = false;
    for i in 2..=15 {
        if list[i] == false {
            continue
        }
        for j in (i*i..200).step_by(i) {
            list[j] = false
        }
    }
    let mut max_length = 0;
    let mut tmp = 0;

    for i in a+c..=b+d {
        if list[i] == true {
            tmp = 0;
        } else {
            tmp += 1;
            max_length = max(max_length, tmp);
        }
    }
    if max_length > d - c {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
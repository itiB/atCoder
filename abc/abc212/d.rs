use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        q: i128,
    }

    let mut bs = BinaryHeap::new();
    let mut diff = 0;

    for _ in 0..q {
        input! {
            n: i128,
        }
        match n {
            1 => {
                input! {
                    x: i128,
                }
                bs.push(Reverse(x - diff));
            },
            2 => {
                input! {
                    x: i128,
                }
                diff += x;
            },
            3 => {
                let Reverse(x) = bs.pop().unwrap();
                println!("{}", x + diff);
            },
            _ => {},
        }
    }
}

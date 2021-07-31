use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        q: i128,
    }

    let mut bs = BTreeSet::new();
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
                bs.insert(x - diff);
            },
            2 => {
                input! {
                    x: i128,
                }
                diff += x;
            },
            3 => {
                let first = bs.iter().next().unwrap().clone();
                println!("{}", first + diff);
                bs.remove(&(first));
            },
            _ => {},
        }
    }
}

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut map: HashSet<usize> = HashSet::new();
    let mut min = 0;

    for num in p {
        map.insert(num);
        loop {
            if !map.contains(&min) {
                break
            }
            min += 1;
        }
        println!("{}", min);
    }
}
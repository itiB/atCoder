use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map: HashMap<usize, usize> = HashMap::new();

    for &a in a.iter() {
        *map.entry(a).or_insert(0) += 1;
    }

    let sum = map
        .values()
        .fold(0, |sum, x| sum + if x > &1 { x * (x - 1) / 2 } else { 0 });

    for &a in a.iter() {
        let n = map.get(&a).unwrap();
        println!("{}", sum - (n - 1));
    }
}

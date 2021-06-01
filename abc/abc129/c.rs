use proconio::input;
use std::collections::HashSet;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }

    let mut map = vec![0; n + 1];
    let hash: HashSet<usize> = a.into_iter().collect();

    map[0] = 1;
    if !hash.contains(&1) {
        map[1] = 1;
    }

    for i in 2..=n {
        if !hash.contains(&i) {
            map[i] = (map[i - 1] + map[i - 2]) % MOD;
        }
    }
    println!("{}", map[map.len() - 1]);
}
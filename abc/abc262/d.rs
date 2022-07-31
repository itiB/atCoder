use proconio::input;
use std::collections::HashMap;
const MOD: i64 = 998244353;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut map: HashMap<usize, usize> = HashMap::new();

    for &a in a.iter() {
        *map.entry(a).or_insert(0) += 1;
    }
    let mut n = vec![0; 101];
    for (k, l) in map {
        let lim = min(100, k);
        for i in 1..=lim {
            n[i] += l;
        }
    }
    let mut ans: i64 = 0;
    for i in 1..=100 {
        ans += combination(n[i] as i64, i as i64);
        ans %= MOD;
    }

    println!("{}", ans);
}

fn combination(mut n: i64, k: i64) -> i64 {
    let mut result = 1;
    for i in 1..=k {
        result = result * n / i;
        n -= 1;
    }
    result
}

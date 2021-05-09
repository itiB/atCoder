use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        k: usize,
        n: i32,
        m: i32,
        a: [i32; k]
    }

    let mut sum = 0;
    let mut map = Vec::new();
    let mut b_map = BTreeMap::new();

    for i in 0..k {
        sum += m * a[i] / n;
        map.push(m * a[i] / n);
        b_map.insert(((m * a[i] / n + 1)*n - a[i]*m).abs(), i);
    }
    sum = m - sum;
    let mut count = 0;
    for (key, value) in b_map.iter_mut() {
        if count == sum { break; }
        map[*value] += 1;
        count += 1;
    }
    for a in map {
        print!("{} ", a);
    }
}
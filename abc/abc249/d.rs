use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    a.sort();
    for &a in a.iter() {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            if a[i] * a[j] > a[n-1] {
                break;
            }
            // println!("a[i] * a[j]: {} x {} = {}", a[i], a[j], a[i]*a[j]);
            match map.get(&(a[i] * a[j])) {
                Some(num) => {
                    if i == j {
                        ans += num;
                    } else {
                        ans += 2 * num;
                    }
                },
                _ => {}
            }
        }
    }
    println!("{}", ans);
}
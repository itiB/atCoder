use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }

    let mut map = HashMap::new();
    for aa in a {
        factor(aa, &mut map);
    }
    let mut ans = vec![1; m + 1];
    ans[0] = 0;
    for k in map.keys() {
        if *k <= m && ans[*k] == 1 {
            for i in (0..=m).step_by(*k) {
                ans[i] = 0;
            }
        }
    }
    let s: usize = ans.iter().sum();
    println!("{}", s);
    for i in 1..ans.len() {
        if ans[i] == 1 {
            println!("{}", i);
        }
    }
}


fn factor(num: usize, map: &mut HashMap::<usize, usize>) {
    let mut n = num;
    let mut i = 2;
    loop {
        if n == 1 {
            break;
        }
        if i * i > num {
            map.insert(n, 1);
            break;
        }
        if n % i == 0 {
            *map.entry(i).or_insert(0) += 1;
            n /= i;
            continue;
        }
        i += 1;
    }
}


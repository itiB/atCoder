use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        xk: [(usize, usize); q]
    }

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let _ = map.entry(a[i]).or_insert(Vec::new());
        map.get_mut(&a[i]).unwrap().push(i + 1);
    }
    for (x, k) in xk {
        if map.contains_key(&x) && map.get(&x).unwrap().len() >=k {
            println!("{}", map.get(&x).unwrap()[k - 1]);
        } else {
            println!("-1");
        }
    }
}
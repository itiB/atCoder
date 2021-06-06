use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for a in 1..=n {
        map.insert(a, Vec::new());
    }
    for (a, b) in ab {
        map.get_mut(&a).unwrap().push(b);
    }

    let mut ans = 0;
    for i in 1..=n {
        let mut past = vec![false; n + 1];
        go(i, &map, &mut past);
        ans += past.iter().filter(|x| **x == true).count();
    }
    println!("{}", ans);
}

fn go(start: usize, map: &HashMap<usize, Vec<usize>>, past: &mut Vec<bool>) {
    if past[start] { return; }
    past[start] = true;
    for m in map.get(&start).unwrap() {
        go(*m, map, past);
    }
}

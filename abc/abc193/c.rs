use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i64,
    }

    let mut set = HashSet::new();
    let mut ans = n;
    let mut i = 2;
    while i * i <= n {
        let mut count = 0;
        let mut prim = i * i;
        while prim <= n {
            if !set.contains(&prim) {
                count += 1;
            }
            set.insert(prim);
            prim *= i;
        }
        ans -= count;
        i += 1;
    }
    println!("{}", ans);
}

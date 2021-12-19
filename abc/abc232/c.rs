use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); m]
    }

    let mut taka = vec![Vec::new(); n];
    let mut aoki = vec![Vec::new(); n];
    for (a, b) in ab {
        taka[a - 1].push(b-1);
        taka[b - 1].push(a-1);
    }
    for (c, d) in cd {
        aoki[c - 1].push(d-1);
        aoki[d - 1].push(c-1);
    }
    for perm in (0..n).permutations(n) {
        for i in 0..n {
            for j in 0..taka[j].len() {
                if j == aoki[perm[i]][]
            }
        }
        // println!("{:?}", perm);
    }
}

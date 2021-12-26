use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); m]
    }

    let mut taka = vec![vec![0; n]; n];
    let mut aoki = vec![vec![0; n]; n];
    for (a, b) in ab {
        taka[a - 1][b - 1] = 1;
        taka[b - 1][a - 1] = 1;
    }
    for (c, d) in cd {
        aoki[c - 1][d - 1] = 1;
        aoki[d - 1][c - 1] = 1;
    }
    let same = (0..n).permutations(n).any(|perm| {
        for i in 0..n {
            for j in 0..n {
                if taka[i][j] != aoki[perm[i]][perm[j]] {
                    return false
                }
            }
        }
        true
    });
    println!("{}", if same {"Yes"} else {"No"});
}

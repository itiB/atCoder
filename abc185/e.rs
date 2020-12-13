use proconio::input;
use std::cmp::min;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut ab = vec![vec![0; m + 1]; n + 1];
    for i in 0..=n {
        ab[i][0] = i;
    }
    for i in 0..=m {
        ab[0][i] = i;
    }
    for y in 0..m {
        for x in 0..n {
            if a[x] != b[y] {
                // ab[x + 1][y + 1] = max(ab[x + 1][y], ab[x][y + 1]);
                ab[x + 1][y + 1] = min(min(ab[x][y] + 1, ab[x + 1][y] + 1), ab[x][y + 1] + 1);
            } else {
                ab[x + 1][y + 1] = ab[x][y]
            }
        }
    }

    // for aa in ab {
    //     println!("{:?}", aa);
    // }
    println!("{}", ab[n][m]);
}
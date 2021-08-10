use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut abcd: [(usize, usize, usize, usize); q]
    }

    // A(len(10))を全探索する
    let mut a = vec![1];

    println!("{}", dfs(&mut a, 0, &abcd, n, m));
}

fn dfs(a: &mut Vec<usize>, mut ans: usize, abcd: &Vec<(usize, usize, usize, usize)>, n: usize, m: usize) -> usize {
    if a.len() == n {
        // check
        let mut tmp = 0;
        for (aa, b, c, d) in abcd {
            if a[*b - 1] - a[*aa - 1] == *c {
                tmp += d;
            }
        }
        ans = max(ans, tmp);
    } else {
        let last_a = a[a.len() - 1];
        for next in last_a..=m {
            a.push(next);
            ans = dfs(a, ans, abcd, n, m);
            a.pop();
        }
    }
    return ans
}
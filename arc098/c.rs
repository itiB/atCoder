use proconio::input;

fn main() {
    input! {
        n: usize,
        mut r: [(usize, usize); n],
        mut b: [(usize, usize); n],
    }
    let mut ans = 0;
    r.sort();
    b.sort_by_key(|tuple| tuple.1);
    for (c, d) in b {
        let mut biggest = None;
        for i in 0..n {
            if r[i].0 < c && r[i].1 < d {
                biggest = Some(i);
            }
        }
        if let Some(i) = biggest {
            r[i] = (201, 201);
            ans += 1;
        }
    }
    println!("{}", ans);
}

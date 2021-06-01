use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    a.dedup();

    let mut ans = a[0];
    for i in 1..a.len() {
        ans = gcd(ans, a[i])
    }
    println!("{}", ans);
}

// 最大公約数を返す
fn gcd(p: usize, q: usize) -> usize {
    if p % q == 0 {
        return q;
    } else {
        return gcd(q, p % q);
    }
}

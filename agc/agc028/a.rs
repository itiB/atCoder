use proconio::input;
use proconio::marker::Chars;
// use std::cmp::max;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }

    for i in 0..gcd(n, m) {
        // 一致すべきタイミングで文字がずれるものを探す
        if s[i * (n / gcd(n , m))] != t[i * (m / gcd(m, n))] {
            println!("-1");
            return;
        }
    }

    // if (0..gcd).all(|i| s[n / gcd * i] == t[m / gcd * i]) {
    //     println!("{}", n * m / gcd);
    // } else {
    //     println!("-1");
    // }

    println!("{}", lcm(n, m));
}

// 最大公約数を返す
fn gcd(p: usize, q: usize) -> usize {
    if p % q == 0 {
        return q;
    } else {
        return gcd(q, p % q);
    }
}

// 最小公倍数を返す
fn lcm(p: usize, q: usize) -> usize {
    return p * q / gcd(p, q);
}

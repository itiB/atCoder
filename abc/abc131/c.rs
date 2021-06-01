use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    println!("{}",
        (b + b/lcm(c, d) - b/c - b/d) - ((a-1) + (a-1)/lcm(c, d) - (a-1)/c - (a-1)/d)
    );
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

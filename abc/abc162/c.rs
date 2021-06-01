use proconio::input;

fn main() {
    input! {
        n: i64
    }

    let mut sum: i64 = 0;
    for a in 1..n + 1 {
        for b in 1..n + 1 {
            for c in 1..n + 1 {
                sum = sum + gcd_in3(a, b, c);
            }
        }
    }
    println!("{}", sum);
}

fn gcd_in3(a: i64, b: i64, c: i64) -> i64 {
    return gcd(gcd(a, b), c);
}

// 最大公約数を返す
fn gcd(p: i64, q: i64) -> i64 {
    if p % q == 0 {
        return q;
    } else {
        return gcd(q, p % q);
    }
}

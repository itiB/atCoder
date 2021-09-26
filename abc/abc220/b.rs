
use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize
    }

    println!("{}", to10(b, k) * to10(a, k));
}

fn to10(x: usize, k: usize) -> usize {
    let mut c = x;
    let mut ans = 0;
    let mut tmp = 1;
    loop {
        ans += c % 10 * tmp;
        c /= 10;
        tmp *= k;
        if c < 10 {
            ans += c * tmp;
            break
        }
    }
    ans
}
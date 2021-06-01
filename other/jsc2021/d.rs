use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i64,
    }

    let mut ans = p - 1;
    for _ in 0..n {
        ans *= p - 1;
        ans /= 1_000_000_007;
    }

    for i in 0..=n {
        
    }

    println!("{}", ans);
}
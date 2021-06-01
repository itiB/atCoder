use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = n;
    for _ in 0..k {
        if ans % 200 == 0 {
            ans /= 200;
        } else {
            ans *= 1000;
            ans += 200;
        }
    }
    println!("{}", ans);
}
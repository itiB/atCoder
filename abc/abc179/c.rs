use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    for i in (1..n).rev() {
        ans += (n - 1) / i;
    }
    println!("{}", ans);
}

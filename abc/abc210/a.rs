use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: usize,
        y: usize,
    }

    let mut ans = 0;
    if n > a {
        ans += a * x;
        ans += (n - a) * y;
    } else {
        ans += n * x;
    }
    println!("{}", ans);
}
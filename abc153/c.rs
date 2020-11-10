use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }

    h.sort();
    let mut ans = 0;
    let t = if n > k { n - k } else { 0 };

    for i in 0..t {
        ans += h[i];
    }
    println!("{}", ans);
}
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    a.push(0);
    a.sort();

    let mut ans = 1;
    for i in 1..=n {
        ans *= a[i] - a[i - 1] + 1;
        ans %= 1_000_000_007;
    }
    println!("{}", ans);
}
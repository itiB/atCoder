use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    let mut ans = 0;
    let mut per = 10;

    if n % 2 != 1 {
        while per <= n {
            ans += n / per;
            per *= 5;
        }
    }

    println!("{}", ans);
}

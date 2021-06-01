use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    }

    if a + 2 >= b {
        println!("{}", k + 1);
    } else {
        let mut ans = a;
        ans += (k - (a - 1)) / 2 * (b - a) + (k - (a - 1)) % 2;
        println!("{}", ans);
    }
}
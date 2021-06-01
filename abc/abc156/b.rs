use proconio::input;

fn main() {
    input! {
        mut n: usize,
        k: usize,
    }

    let mut ans = 1;
    loop {
        if n < k {
            break;
        }
        n /= k;
        ans += 1;
    }
    println!("{}", ans);
}
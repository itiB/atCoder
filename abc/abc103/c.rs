use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut ans = 0;
    for i in a {
        ans += i - 1;
    }
    println!("{}", ans);
}
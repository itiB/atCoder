use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut ans = 0;
    for t in a {
        if t > 10 {
            ans += t - 10;
        }
    }
    println!("{}", ans);
}
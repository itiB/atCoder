use proconio::input;

fn main() {
    input! {
        n:usize,
        k: usize,
    }

    let mut ans = 0;
    for i in 1..n+1 {
        ans += 100 * i * k;
    }
    for i in 1..k + 1 {
        ans += i * n;
    } 
    println!("{}", ans);
}
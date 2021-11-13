use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize
    }

    let mut ans = (a+k-1)%n;
    if ans == 0 { ans = n }
    println!("{}", ans);
}
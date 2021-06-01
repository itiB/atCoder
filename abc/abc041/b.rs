use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    let mut ans = a;
    ans = ans * b % 1000000007;
    ans = ans * c % 1000000007;
    println!("{}", ans);
}
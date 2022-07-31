use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut ans: i64 = 0;
    let mut tmp: i64 = 0;
    for v in 1..=n {
        if v == a[v - 1] {
            tmp += 1;
        }
        if v < a[v - 1] && a[a[v - 1] - 1] == v {
            ans += 1;
        }
    }
    ans += tmp * (tmp - 1) / 2;
    println!("{}", ans);
}

use proconio::input;

fn main() {
    input! {
        mut h: usize,
    }

    let mut ans: usize = 0;
    let mut count: usize = 1;

    while h > 0 {
        h /= 2;
        ans += count;
        count *= 2;
    }
    println!("{}", ans);
}

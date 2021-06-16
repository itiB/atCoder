use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut ans = 0.0;
    for i in 1..=n {
        let mut tmp = n;
        let mut thr = i;
        while thr < k {
            tmp *= 2;
            thr *= 2;
        }
        ans += 1.0 / tmp as f64;
    }
    println!("{}", ans);
}
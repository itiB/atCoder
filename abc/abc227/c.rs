use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for a in 1..n {
        for b in a..=n {
            if a * b * b > n { break }
            let max_c = n / (a * b);
            ans += 1;
        }
    }
    println!("{}", ans);
}
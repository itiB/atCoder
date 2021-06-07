use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); n]
    }

    ab.sort_by_key(|a| a.0);
    let mut ans = 0;
    let mut count = 0;
    for (a, b) in ab {
        for _ in 0..b {
            ans += a;
            count += 1;
            if count == m { break }
        }
        if count == m { break }
    }
    println!("{}", ans);
}
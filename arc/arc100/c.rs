use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n]
    }

    for i in 0..n {
        a[i] = a[i] - i as i64 - 1;
    }
    a.sort();
    let b = if n % 2 == 1 {
        a[n / 2]
    } else {
        (a[n / 2] + a[n / 2 - 1]) / 2
    };
    let mut ans = 0;
    for m in a {
        ans += (m - b).abs();
    }
    println!("{}", ans);
}
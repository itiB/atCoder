use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n]
    }

    let mut sum = 0.0;
    for (a, b) in &ab {
        sum += a / b;
    }
    let mid = sum / 2.0;
    sum = 0.0;
    let mut ans = 0.0;
    for (a, b) in &ab {
        if sum + a / b >= mid {
            // 越えた！！
            ans += (mid - sum) * b;
            break
        } else {
            sum += a / b;
            ans += a;
        }
    }
    println!("{}", ans);
}
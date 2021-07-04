use proconio::input;

fn main() {
    input! {
        mut p: usize,
    }

    let mut v = Vec::new();
    let mut tmp = 1;
    for i in 1..=10 {
        tmp *= i;
        v.push(tmp);
    }
    let mut ans = 0;
    for y in v.iter().rev() {
        if p >= *y {
            ans += p / y;
            p %= y;
        }
    }
    println!("{}", ans);
}
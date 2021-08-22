use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    let mut now = 1;
    loop {
        if now <= n {
            ans += 1;
            now *= 2;
        } else {
            break
        }
    }
    println!("{}", ans - 1);
}
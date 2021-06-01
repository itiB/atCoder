use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 1;
    let mut time = 10;
    while ans + ans * time <= n {
        ans += 1;
        if ans >= time {
            time *= 10;
        }
    }
    println!("{}", ans - 1);
}

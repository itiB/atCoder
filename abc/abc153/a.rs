use std::io;

fn main() {
    let vec = read_vec::<i32>();

    let h: i32 = vec[0];
    let a: i32 = vec[1];

    let mut ans: i32 = h / a;
    if h % a != 0 {
        ans = ans + 1;
    }
    println!("{}", ans);
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .ok()
        .expect("Faild to read Vec");
    line.trim().split_whitespace()
        .map(|i| i.parse().ok().unwrap()).collect()
}
use std::io;

fn main() {
    let words = read_vec::<String>();
    let mut num   = read_vec::<i32>();

    if words[0] == read::<String>() {
        num[0] = num[0] - 1;
    } else {
        num[1] = num[1] - 1;
    }
    println!("{} {}", num[0], num[1]);
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .ok()
        .expect("Faild to read Vec");
    line.trim().split_whitespace()
        .map(|i| i.parse().ok().unwrap()).collect()
}

fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}

use std::io;

fn main() {
    let status = read_vec::<i32>();
    let spattack = read_vec::<i32>();

    let h: i32 = status[0];

    let mut sum: i32 = 0;
    for num in spattack{
        sum = sum + num;
    }

    if sum >= h {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .ok()
        .expect("Faild to read Vec");
    line.trim().split_whitespace()
        .map(|i| i.parse().ok().unwrap()).collect()
}
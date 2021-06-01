fn main() {
    let nm = read_vec::<i64>();
    let mut n = nm[0];
    let m = nm[1];

    n = n % m;

    loop {
        if n > (n - m).abs() {
            n = (n - m).abs();
        } else {
            break;
        }
    }
    println!("{}",n);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

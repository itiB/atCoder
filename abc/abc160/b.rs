fn main() {
    let n = read::<i32>();
    println!("{}", (n / 500) * 1000 + (n % 500 / 5) * 5);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

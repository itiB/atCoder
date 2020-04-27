fn main() {
    let s: Vec<char> = read::<String>().chars().collect();
    if s[2] == s[3] && s[4] == s[5] {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

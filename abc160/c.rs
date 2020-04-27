fn main() {
    let kn = read_vec::<i32>();
    let a = read_vec::<i32>();

    let mut length = Vec::new();
    for a_next in 1..(kn[1]) {
        length.push(a[a_next as usize] - a[a_next as usize - 1]);
    }
    length.push(kn[0] + a[0] - a[kn[1] as usize - 1]);
    println!("{}", length.iter().sum::<i32>() - length.iter().max().unwrap());
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

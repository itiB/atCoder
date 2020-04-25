fn main() {
    let nm = read_vec::<usize>();
    let mut a_vec = read_vec::<i32>();
    let n: usize = nm[0];
    let m: usize = nm[1];
    let mut sum: i32 = 0;

    a_vec.sort();
    a_vec.reverse();

    for number in 0..n {
        sum = sum + a_vec[number];
    }
    if m > n {
        println!("No");
    } else {
        if a_vec[m - 1] as f32 >= sum as f32/ (4.0 * m as f32) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
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

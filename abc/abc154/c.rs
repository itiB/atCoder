use std::io;

fn main() {
    let n: i32 = read::<i32>();
    let mut a_vec = read_vec::<i32>();

    // for i in 0..(n as usize) {
    //     let mut target: i32 = a_vec[i];
    //     for j in (i + 1)..(n as usize) {
    //         if a_vec[j] == target {
    //             println!("NO");
    //             return;
    //         }
    //     }
    // }
    // println!("YES");
    a_vec.sort();
    a_vec.dedup();

    if n != a_vec.len() as i32 {
        println!("NO")
    } else {
        println!("YES");
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .ok()
        .expect("Faild to read Vec");
    line.trim().split_whitespace()
        .map(|i| i.parse().ok().unwrap()).collect()
}

use std::io;

fn main() {
    let a_vec = read_vec::<i32>();

    // for c in &a_vec {
    //     if c == &7 {
    //         println!("Yes");
    //     }
    // }

    if a_vec.contains(&7) {
        println!("Yes!");
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

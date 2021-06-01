use std::io;

fn main() {
    let vec = read_vec::<i32>();

    if let(1...9, 1...9) = (vec[0], vec[1]){
        println!("{}", vec[0] * vec[1])
    } else {
        println!("-1")
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

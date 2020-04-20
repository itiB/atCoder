use std::io;

fn main() {
    let mut line = read::<String>();

    let len: usize = line.len();

    for _i in 0..len {
        line.pop();
    }
    for _i in 0..len {
        line.push('x');
    }
    println!("{}", line);
}

fn read<T: std::str::FromStr>() -> T {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}

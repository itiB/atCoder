use std::io;

fn main() {
	let vec = read_vec::<i32>();
	let h_vec = read_vec::<i32>();

	let k: i32 = vec[1];

	let count = h_vec.into_iter().filter(|&a| a >= k).count();

	println!("{}", count);
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
	let mut line = String::new();
	io::stdin().read_line(&mut line)
		.ok()
		.expect("Faild to read Vec");
	line.trim().split_whitespace()
		.map(|i| i.parse().ok().unwrap()).collect()
}
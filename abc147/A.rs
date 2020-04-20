use std::io;

fn main() {
	let mut num = String::new();

	io::stdin().read_line(&mut num)
		.ok()
		.expect("Failed to read line");

	let number: i32 = num.trim().parse().ok().unwrap();

	if number % 2 == 0 {
		println!("0.5");
	} else {
		let answer: f32 = ((number / 2 + 1) as f32) / number as f32;
		println!("{}", answer);
	}
}

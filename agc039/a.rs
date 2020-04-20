use std::io;

fn main() {
	let s = read::<String>();
	let k = read::<i64>();

	// 入力された文字列を2回繰り返した文字列の作成
	let mut count: i64 = 0;

	// 初回の値を取得
	let (num_1, last_char) = ret_char_num(&s);

	// 偶数回目の値を取得
	let first_s: String = format!("{}{}", last_char, s);
	let (num_2, last_char) = ret_char_num(&first_s);

	// 奇数回目の値を取得
	let mids_s: String = format!("{}{}", last_char, s);
	let (num_3, _last_char) = ret_char_num(&mids_s);

	if k > 0{
		count = num_1 + (num_2 + num_3) * (k / 2);
		if k % 2 == 0 {
			count = count - num_3;
		}
	}
	println!("{}", count);
}

fn read<T: std::str::FromStr>() -> T {
	let mut line = String::new();
	io::stdin().read_line(&mut line).ok();
	line.trim().parse().ok().unwrap()
}

fn ret_char_num(s : &String) -> (i64, char) {
	// 渡された文字列の連続する文字の数を返す

	let mut vec: Vec<char> = s.as_str().chars().collect();
	let mut count: i64 = 0;

	for i in 1..vec.len(){
		// 1つ前と一致した場合，存在しない文字に書き換える
		if vec[i - 1] == vec[i]{
			count = count + 1;
			vec[i] = '!';
		}
	}
	return (count, vec[vec.len() - 1]);
}
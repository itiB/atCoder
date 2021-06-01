use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut sum = 0;
    for i in 0..1 << s.len() - 1 {
        // 間のパターンを列挙する
        let mut counter = 0;
        for j in (0..s.len()).rev() {
            // 右から順番に数字を足していく
            if i >> j & 1 == 1 {
                // 足し算マークがついているならば10乗を0にする
                counter = 0;
            }
            sum += num(s[j]) * 10_usize.pow(counter as u32);
            counter += 1;
        }
    }
    println!("{}", sum);
}

fn num(c: char) -> usize {
    c as usize - 48
}
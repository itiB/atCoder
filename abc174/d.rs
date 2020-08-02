use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        mut c: Chars,
    }

    let mut count = 0;
    let mut i = 0; // 左端
    let mut j = c.len() - 1; // それ以上先にRがない右端

    while i < j && i < c.len() - 1{
        // もし左端がWでなければ後ろから順にRを探しに行く
        if c[i] == 'W' {
            while j > i {
                if c[j] == 'R' {
                    c[i] = 'R';
                    c[j] = 'W';
                    count += 1;
                    break;
                }
                j -= 1;
            }
        }
        i += 1;
    }
    println!("{}", count);
}

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }

    let mut strs = Vec::new();
    for i in 0..s.len() {
        let mut c = Vec::new();
        for j in 0..s.len() {
            c.push(s[(j + i) % s.len()]);
        }
        let word: String = c.iter().collect();
        strs.push(word);
    }
    strs.sort();
    println!("{}", strs[0]);
    println!("{}", strs[strs.len() - 1]);
}
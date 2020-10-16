use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        ss: Chars,
        t: Chars
    }

    if ss.len() >= t.len() {
        for i in (0..=ss.len() - t.len()).rev() {
            for j in 0..t.len() {
                if ss[i + j] != t[j] && ss[i + j] != '?' {
                    break;
                }
                if j == t.len() - 1 {
                    // 最後までこれた文字列
                    for v in 0..i {
                        print!("{}", if ss[v] == '?' { 'a' } else { ss[v] })
                    }
                    for v in 0..t.len() {
                        print!("{}", t[v]);
                    }
                    for v in i + t.len()..ss.len() {
                        print!("{}", if ss[v] == '?' { 'a' } else { ss[v] })
                    }
                    println!("");
                    return
                }
            }
        }
    }
    println!("UNRESTORABLE");
}
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    }

    let mut ans = 2 * n;
    for i in 0..n {
        let mut flag = true; 
        for j in i..n {
            if s[j] != t[j - i] {
                flag = false;
            }
        }
        if flag == true {
            ans = n + i;
            break;
        }
    }
    println!("{}", ans);
}
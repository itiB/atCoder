use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let l = s.len();
    let mut ans = 0;
    for i in 0..l {
        if s[i] == 'U' {
            ans += l - 1 - i;
            ans += i * 2;
        } else {
            ans += (l - 1 - i) * 2;
            ans += i;
        }
    }
    println!("{}", ans);
}
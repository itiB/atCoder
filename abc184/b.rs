use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        x: i32,
        s: Chars,
    }

    let mut ans = x;
    for c in s {
        match c {
            'o' => ans += 1,
            _ => {
                ans -= 1;
                if ans < 0 {
                    ans = 0;
                }
            }
        }
    }
    println!("{}", ans);
}
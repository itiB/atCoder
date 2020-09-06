use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: usize,
        s: usize,
        p: usize,
        mut t: Chars,
    }

    let mut ans = 0;

    for num in 0..n {
        if num >= k {
            if t[num] != t[num - k] {
                match t[num] {
                    'r' => ans += p,
                    's' => ans += r,
                    'p' => ans += s,
                    _ => {},
                }
            } else {
                t[num] = 'c';
            }
        } else {
            match t[num] {
                'r' => ans += p,
                's' => ans += r,
                'p' => ans += s,
                _ => {},
            }
        }
    }

    println!("{}", ans);
}
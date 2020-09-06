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
        if num >= k && t[num] == t[num - k] {
            t[num] = 'c';
        }
        ans += match t[num] {
            'r' => p,
            's' => r,
            'p' => s,
            _ => 0,
        }
    }

    println!("{}", ans);
}
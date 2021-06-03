use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        c: Chars,
    }

    let mut left = 0;
    let mut right = n - 1;
    let mut ans = 0;
    while left < right && left < n {
        if c[left] == 'W' {
            while right > left {
                if c[right] == 'R' {
                    ans += 1;
                    right -= 1;
                    break;
                }
                right -= 1;
            }
        }
        left += 1;
    }
    println!("{}", ans);
}
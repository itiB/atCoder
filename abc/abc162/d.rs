use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut r: usize = 0;
    let mut g: usize = 0;
    let mut b: usize = 0;

    for c in &s {
        match c {
            'R' => r += 1,
            'G' => g += 1,
            'B' => b += 1,
            _ => unreachable!(),
        }
    }

    // 全部違う文字を選ぶ
    let mut ans = r * g * b;

    // i, j, k が等間隔ならば
    // j - i == k - j は k = 2j - i
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            let k = 2 * j - i;
            if k < n {
                // i, j, kが等間隔かつすべての文字が異なるならば
                if s[i] != s[j] && s[j] != s[k] && s[k] != s[i] {
                    ans -= 1;
                }
            }
        }
    }
    println!("{}", ans);
}
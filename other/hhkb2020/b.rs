use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ans = 0;

    for y in 0..h - 1 {
        for x in 0..w - 1 {
            if s[y][x] == '.' && s[y + 1][x] == '.' {
                ans += 1;
            }
            if s[y][x] == '.' && s[y][x + 1] == '.' {
                ans += 1;
            }
        }
    }
    for x in 0..w - 1 {
        if s[h - 1][x] == '.' && s[h - 1][x + 1] == '.' {
            ans += 1;
        }
    }
    for y in 0..h - 1 {
        if s[y][w - 1] == '.' && s[y + 1][w - 1] == '.' {
            ans += 1;
        }
    }
    println!("{}", ans);
}
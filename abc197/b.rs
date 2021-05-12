use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        y: usize,
        s: [Chars; h],
    }
    let mut ans = if s[x - 1][y - 1] == '#' { 0 } else { 1 };
    // うえ
    for i in (1..x).rev() {
        if s[i - 1][y - 1] == '.' {
            ans += 1;
        } else {
            break;
        }
    }
    // した
    for i in x+1..=h {
        if s[i - 1][y - 1] == '.' {
            ans += 1;
        } else {
            break;
        }
    }
    // ひだり
    for i in (1..y).rev() {
        if s[x - 1][i - 1] == '.' {
            ans += 1;
        } else {
            break;
        }
    }
    // みぎ
    for i in y+1..=w {
        if s[x - 1][i - 1] == '.' {
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
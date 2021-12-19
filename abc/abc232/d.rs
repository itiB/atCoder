use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }

    let mut map = vec![vec![0; w]; h];
    map[0][0] = 1;
    let mut ans = 1;
    for y in 0..h {
        for x in 0..w {
            if c[y][x] == '.' {
                let mut new = map[y][x];
                if y > 0 {
                    if map[y-1][x] != 0 {
                        new = max(new, map[y - 1][x] + 1);
                    }
                }
                if x > 0 {
                    if map[y][x-1] != 0 {
                        new = max(new, map[y][x - 1] + 1);
                    }
                }
                map[y][x] = new;
                ans = max(ans, new);
            }
        }
    }
    println!("{}", ans);
}
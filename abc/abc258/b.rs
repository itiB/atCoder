use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a: [Chars; n]
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            // (i, j)について8方向で選んでいく
            ans = max(ans, cal(1, 0, i, j, &a));
            ans = max(ans, cal(0, 1, i, j, &a));
            ans = max(ans, cal(-1, 0, i, j, &a));
            ans = max(ans, cal(0, -1, i, j, &a));
            ans = max(ans, cal(1, 1, i, j, &a));
            ans = max(ans, cal(-1, -1, i, j, &a));
            ans = max(ans, cal(1, -1, i, j, &a));
            ans = max(ans, cal(-1, 1, i, j, &a));
        }
    }
    println!("{}", ans);
}

fn cal(x: i32, y: i32, i: usize, j: usize, map: &Vec<Vec<char>>) -> usize {
    // 進む方向とスタート地点を渡していくつになるか計算する
    let mut ans = 0;
    let mut t = 10_usize.pow(map.len() as u32 - 1);
    let mut xx = i;
    let mut yy = j;
    for _ in 0..map.len() {
        ans += (map[xx][yy] as i32 - 48) as usize * t;
        t /= 10;
        xx = if xx as i32 + x < 0 {
            map.len() - 1
        } else {
            (xx as i32 + x) as usize
        };
        if xx == map.len() {
            xx = 0
        }
        yy = if yy as i32 + y < 0 {
            map.len() - 1
        } else {
            (yy as i32 + y) as usize
        };
        if yy == map.len() {
            yy = 0
        }
    }
    return ans;
}

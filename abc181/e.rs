use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [i64; n],
        mut w: [i64; m],
    }

    h.sort();
    w.push(std::i64::MAX / 2);
    w.push(std::i64::MIN / 2);
    w.sort();

    let mut ds = vec![0; n]; // 指定した値までの身長差合計
    let mut de = vec![0; n]; // 指定した値以降の身長差合計

    let mut now = 0;
    for i in 1..n {
        if i % 2 == 1 {
            now += h[i] - h[i - 1];
        }
        ds[i] = now;
    }
    let mut now = 0;
    for i in (0..n).rev() {
        if i % 2 == 1 {
            now += h[i + 1] - h[i];
        }
        de[i] = now;
    }

    let mut ans = std::i64::MAX;

    for i in 0..n {
        // 偶数番目は結局奇数番目と同じことをすることになる
        let mut now = 0;
        let j = teacher_h(h[i], &w);
        now += min(w[j] - h[i], h[i] - w[j - 1]);
        now += ds[i] + de[i];
        ans = min(ans, now);
    }
    println!("{}", ans);
}

fn teacher_h(student: i64, teacher: &[i64]) -> usize {
    // 最も対象生徒に近い身長の先生を2分探索で求める
    let mut l = 0;
    let mut r = teacher.len() - 1;
    let mut m = (teacher.len() - 1) / 2;
    while l < m {
        if student > teacher[m] {
            l = m;
        } else {
            r = m;
        }
        m = (l + r) / 2;
    }
    r
}
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n]
    }
    let mut cw: Vec<usize> = Vec::new();
    let mut aw: Vec<usize> = Vec::new();
    for i in 0..s.len() {
        if s[i] == '0' {
            cw.push(w[i]);
        } else {
            aw.push(w[i]);
        }
    }
    cw.sort_unstable();
    aw.sort_unstable();
    if cw.is_empty() || aw.is_empty() {
        println!("{}", n);
        return;
    }

    let mut fail_child = cw.len(); // 一番初めの子供から大人だと思われている
    let mut fail_adult = 0; // 全員おとなだと思われている
    let mut min_fail_sum = fail_child;
    let mut i = 1;
    loop {
        if i > max(cw[cw.len() - 1], aw[aw.len() - 1]) {
            break;
        }
        // i 以上ならば大人だと考える
        // println!("{} fail_c: {}, fail_a: {}", i, fail_child, fail_adult);
        if fail_child != 0 && cw[cw.len() - fail_child] == i {
            fail_child -= 1;
        } else if fail_adult != aw.len() && aw[fail_adult] == i {
            fail_adult += 1;
        } else {
            i += 1;
            min_fail_sum = min(min_fail_sum, fail_adult + fail_child);
        }
    }
    println!("{}", n - min_fail_sum);
}

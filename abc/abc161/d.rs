use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        k: usize
    }

    let mut lun = VecDeque::new();
    let mut lunlun = Vec::new();
    for i in 1..=9 {
        inc(&mut lun, i, &mut lunlun);
    }
    lunlun.sort();
    println!("{}", lunlun[k - 1]);
}

fn inc(a: &mut VecDeque<i64>, n: i64, lunlun: &mut Vec<i64>) {
    a.push_back(n);

    // aを数字にしてlunlunに入れる。
    let mut lun = 0;
    let mut c = 1;
    for i in (0..a.len()).rev() {
        lun += a[i] * c;
        c *= 10;
    }
    lunlun.push(lun);

    if a.len() <= 10 {
        // まだ伸ばせるなら次へ
        for i in -1..=1 {
            if 0 <= n + i && n + i < 10 {
                inc(a, n + i, lunlun);
            }
        }
    }
    a.pop_back();
}
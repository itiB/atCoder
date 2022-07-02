use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n]
    }
    let mut mid: Vec<i128> = Vec::new();
    mid.push((ab[0].0 + ab[0].1) as i128);
    for i in 1..n {
        mid.push(mid[i - 1] + (ab[i].0 + ab[i].1) as i128);
    }
    if x == 1 {
        println!("{}", mid[0]);
        return;
    }
    let mut ans: i128 = std::i128::MAX;
    for i in 0..n {
        // 後ろからi番目のステージをたくさんこなす
        let creared = n - i;
        let need = x - creared;
        let time = mid[creared - 1] + (need * ab[creared - 1].1) as i128;
        ans = min(ans, time)
    }
    println!("{}", ans);
}

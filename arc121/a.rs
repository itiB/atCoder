use proconio::input;
use std::cmp::max;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n]
    }

    let mut ids: Vec<(i64, i64)> = Vec::new();
    xy.sort_by_key(|a| a.0);
    ids.push(xy[0]);
    ids.push(xy[1]);
    if xy.len() > 2 { ids.push(xy[xy.len() - 1]) };
    if xy.len() > 3 { ids.push(xy[xy.len() - 2]) };

    let last = max(xy.len() - 2, 2);
    let mut v: Vec<(i64, i64)> = xy.drain(2..last).collect();
    v.sort_by_key(|a| a.1);
    if v.len() > 0 { ids.push(v[0]) };
    if v.len() > 1 { ids.push(v[1]) };
    if v.len() > 2 { ids.push(v[v.len() - 1]) };
    if v.len() > 3 { ids.push(v[v.len() - 2]) };

    let mut ans = Vec::new();
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            ans.push(max((ids[i].0 - ids[j].0).abs(), (ids[i].1 - ids[j].1).abs()));
        }
    }
    ans.sort_by_key(|&x| Reverse(x));
    println!("{}", ans[1]);
}

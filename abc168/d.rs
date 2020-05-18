use proconio::input;
use std::collections::VecDeque;

const INF: usize = 1001001001;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a_b: [(usize, usize); m],
    }

    // 隣接リストを作る
    // 0からつながるところ
    // 1からつながるところ
    // ...
    let mut to = vec![Vec::new(); n];
    for (a, b) in a_b {
        to[a - 1].push(b - 1);
        to[b - 1].push(a - 1);
    }

    let mut q = VecDeque::<usize>::new();
    let mut dist = vec![INF; n];
    let mut pre = vec![INF; n];

    // 始点の設定
    dist[0] = 0;
    q.push_front(0);

    // 始点をqから取り出してその隣をキューに追加し更新していく
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        for target in to[v].iter() {
            // すでにDistに値が入っていたならば
            if dist[*target] != INF { continue; }
            // 到達していないところならば距離と直前の番号を記録していく
            dist[*target] = dist[v] + 1;
            pre[*target] = v;
            // qにPush
            q.push_back(*target);
        }
    }

    println!("Yes");
    for i in 1..n {
        let mut ans = pre[i];
        ans += 1;
        println!("{}", ans);
    }
    print!("\n");
}

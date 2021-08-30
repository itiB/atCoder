use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m]
    }
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for (x, y) in xy {
        graph[x].push(y);
    }

    let mut seen: Vec<bool> = vec![false; n+1];
    let mut order: Vec<usize> = Vec::new();
    let mut dp: Vec<usize> = vec![0; n+1];
    let mut ans = 0;
    for v in 1..=n {
        ans = max(ans, rec(&graph, v, &mut seen, &mut order, &mut dp));
    }
    println!("{}", ans);
}

/// 深さ優先探索
fn rec(graph: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>, order: &mut Vec<usize>, dp: &mut Vec<usize>) -> usize {
    // もしすでにDP取っている頂点ならばその値を取得
    if seen[v] { return dp[v] }
    seen[v] = true;
    // v から行ける各頂点 next_v について
    for next_v in &graph[v] {
        // 行ける先の中で最も長い距離持っているものを取得
        dp[v] = max(dp[v], rec(graph, *next_v, seen, order, dp) + 1);
    }
    order.push(v);
    dp[v]
}

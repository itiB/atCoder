use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(i32, i32); m]
    }

    let mut seen: Vec<bool> = vec![false; n+1];
    let mut finished: Vec<bool> = vec![false; n+1];
    let mut order: Vec<i32> = Vec::new();
    let mut graph: Vec<Vec<i32>> = vec![Vec::new(); n+1];
    for (a, b) in ab {
        graph[a as usize].push(b);
    }

    for v in (1..=n).rev() {
        if seen[v] { continue; }
        if !rec(&graph, v as i32, -1, &mut seen, &mut finished, &mut order) {
            println!("-1");
            return
        }
    }
    for i in (1..order.len()).rev() {
        print!("{} ", order[i]);
    }
    println!("{}", order[0]);
}

/// ループ検出つきトポロジカルソート
/// p: vの親
fn rec(graph: &Vec<Vec<i32>>, v: i32, p: i32, seen: &mut Vec<bool>, finished: &mut Vec<bool>, order: &mut Vec<i32>) -> bool {
    seen[v as usize] = true;
    let mut cp = graph[v as usize].clone();
    cp.sort_by_key(|&x| Reverse(x));
    for next_v in cp {
        if finished[next_v as usize] { continue } // 完全終了した頂点はスルー
        if seen[next_v as usize] && !finished[next_v as usize]{
            // サイクルの検出
            return false
        }
        if next_v == p { continue } // 探索が親方向に逆流することを防ぐ
        if !rec(graph, next_v, v, seen, finished, order) {
            return false
        }
    }
    order.push(v);
    finished[v as usize] = true;
    true
}

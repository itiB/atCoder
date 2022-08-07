use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize, // 頂点
        m: usize  // 辺の数
    }

    // グラフは移動状態を含めた頂点を増やす
    // すでに移動した距離が0の頂点、1の頂点、2の頂点が存在
    // それぞれ頂点番号+n*xで表す
    let mut graph = Graph::new(n * 3);

    for _ in 0..m {
        input! {
            u: usize,
            v: usize
        }
        // 移動距離0から1に
        graph.edges[u - 1].push(Edge {
            to: n + v - 1,
            weight: 1,
        });
        // 移動距離1から2になるパターン
        graph.edges[n + u - 1].push(Edge {
            to: 2 * n + v - 1,
            weight: 1,
        });
        // 移動距離2から0になるパターン
        graph.edges[n * 2 + u - 1].push(Edge {
            to: v - 1,
            weight: 1,
        });
    }
    input! {
        s: usize, // 始点
        t: usize  // 終点
    }
    match graph.bfs(s - 1)[t - 1] {
        Some(a) => println!("{}", a / 3),
        None => println!("-1"),
    }
}

/// 辺
#[derive(Debug)]
struct Edge {
    to: usize,
    weight: i64,
}

#[derive(Debug)]
struct Graph {
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        let mut edges = Vec::new();
        for _ in 0..n {
            edges.push(Vec::new())
        }
        Graph { edges }
    }

    fn bfs(self, s: usize) -> Vec<Option<usize>> {
        let n = self.edges.len(); // 頂点数
        let mut dist = vec![None; n];
        let mut q = VecDeque::new();

        // 初期条件
        dist[s] = Some(0);
        q.push_back(s);

        // BFS開始
        while let Some(v) = q.pop_front() {
            // vからたどれる頂点をすべてたどる
            for x in &self.edges[v] {
                // すでに発見済みの頂点は探索しない
                if dist[x.to].is_some() {
                    continue;
                }
                // 新たな頂点xについて距離情報を更新してキューに入れる
                if let Some(dist_v) = dist[v] {
                    dist[x.to] = Some(dist_v + 1);
                    q.push_back(x.to);
                }
            }
        }
        return dist;
    }
}

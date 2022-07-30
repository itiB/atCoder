use proconio::input;

fn main() {
    input! {
        n: usize, // 頂点
        m: usize  // 辺の数
    }
    let mut graph = Graph::new(n);

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
            c: i64
        }
        graph.edges[a - 1].push(Edge {
            to: b - 1,
            weight: -c,
        });
    }

    // ベルマン・フォード法
    let mut exist_negative_cycle = false;
    let mut dist = vec![std::i64::MAX; n];
    dist[0] = 0;

    let mut update = false; // 更新発生したかフラグ
    for iter in 0..n {
        for v in 0..n {
            // まだ緩和されていない点は頂点からの緩和を行わない
            if dist[v] == std::i64::MAX {
                continue;
            }

            for e in &graph.edges[v] {
                // 緩和処理
                if dist[e.to] > dist[v] + e.weight {
                    dist[e.to] = dist[v] + e.weight;
                    update = true;
                    if iter == n - 1 && e.to == n - 1 {
                        exist_negative_cycle = true
                    }
                }
            }
            if !update {
                break;
            } // 更新がなければ最短がすでに求まっている
        }
    }

    if exist_negative_cycle {
        println!("inf");
    } else {
        println!("{}", -dist[n - 1]);
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
}

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize, // 頂点
        m: usize  // 辺の数
    }
    let mut graph = Graph::new(n);

    for _ in 0..m {
        input! {
            u: usize,
            v: usize
        }
        graph.edges[u - 1].insert(v - 1);
        graph.edges[v - 1].insert(u - 1);
    }
    let mut ans = 0;
    for a in 0..n {
        for b in a..n {
            for c in b..n {
                if graph.edges[a].contains(&(b))
                    && graph.edges[b].contains(&(c))
                    && graph.edges[c].contains(&(a))
                {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}

/// 辺
#[derive(Debug)]
struct Edge {
    to: usize,
    weight: i64,
}

#[derive(Debug)]
struct Graph {
    edges: Vec<HashSet<usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        let mut edges = Vec::new();
        for _ in 0..n {
            edges.push(HashSet::new())
        }
        Graph { edges }
    }
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; n],
        cd: [(usize, usize); m]
    }

    let mut ab = Vec::new();
    for i in 0..n {
        ab.push(a[i] - b[i]);
    }
    let mut uf = UnionFind::new(n);
    
    for pair in cd {
        uf.unite(pair.0 - 1, pair.1 - 1);
    }
    let mut counter = vec![0; n];
    for i in 0..n {
        if uf.rank[i] < 0 {
            counter[i] += ab[i];
        } else {
            counter[uf.root(i) as usize] += ab[i];
        }
    }
    for num in counter {
        if num != 0 {
            println!("No");
            return
        }
    }
    println!("Yes");
}

#[derive(Debug)]
struct UnionFind {
    // 自身が親ならばその集合に属する頂点数に -1 をかけたもの
    // そうでなければ親のid
    rank: Vec<i32>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            // 各根は最初それぞれ独立なため -1 で初期化される
            rank: vec![-1; n],
        }
    }

    // 根を求める
    fn root(&mut self, x: usize) -> usize {
        if self.rank[x] < 0 { return x }
        // 求めたノードのroot情報を更新する
        self.rank[x] = self.root(self.rank[x] as usize) as i32;
        self.rank[x] as usize
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        // 各木の根を求める
        let x_root = self.root(x);
        let y_root = self.root(y);
        // 根が一致するならもうすでに数えられているため計算しないfalse
        if x_root == y_root {
            return false;
        }
        // 二つの根を合成する
        // すでに多くの子を持っているものをxとしておく
        // のちにxを根として合成していく
        if self.rank[x_root] > self.rank[y_root] {
            self.rank.swap(x_root, y_root);
        }
        // 各根の保持するノード数を合計してして数を保存
        self.rank[x_root] += self.rank[y_root];
        // yのルートを更新
        self.rank[y_root] = x_root as i32;
        true
    }
}
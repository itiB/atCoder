use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut ans = 0;
    let mut all = UnionFind::new(n);
    for (a, b) in &ab {
        all.unite(*a-1, *b-1);
    }
    let all_size = all.size(0);

    for i in 0..m {
        let mut without = UnionFind::new(n);
        for j in 0..m {
            if i == j { 
                continue; }
            without.unite(ab[j].0 - 1, ab[j].1 - 1);
        }
        if without.size(0) != all_size { ans += 1; }
    }
    println!("{}", ans);
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

    fn size(&mut self, x: usize) -> i32 {
        let root = self.root(x);
        -1 * self.rank[root]
    }
}

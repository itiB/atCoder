use proconio::input;

fn main() {
    input! {
        m: usize,
        uv: [(usize, usize); m],
        p: [usize; 8]
    }

    let mut uf = UnionFind::new(10);
    for (u, v) in uv {
        uf.unite(u, v);
    }
    let mut need_change = Vec::new();
    for i in 0..8 {
        if !uf.same(p[i], i+1) {
            println!("-1");
            return
        }
        if p[i] != i+1 {
            need_change.push(i);
        }
    }
    if need_change.len() == 0 {
        println!("0");
        return
    }
    // すべて同じ島で無いと-1
    for i in 1..need_change.len() {
        if !uf.same(p[need_change[0]], p[need_change[i]]) {
            println!("-1");
            return
        }
    }
    let mut nodes = Vec::new();
    let root = uf.root(p[need_change[0]]);
    let mut noned = false;
    for i in 1..=9 {
        if uf.same(root, i) {
            let mut a: Option<usize> = None;
            for j in 0..8 {
                if p[j] == i {
                    a = Some(j+1);
                }
            }
            if a == None { noned = true }
            nodes.push((i, a));
        }
    }
    if noned == false {
        // 島に空きが1つないと-1
        println!("-1");
        return
    }
    println!("nodes: {:?}", nodes);
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

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn size(&mut self, x: usize) -> i32 {
        let root = self.root(x);
        -1 * self.rank[root]
    }
}

use proconio::input;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        m: usize,
        uv: [(usize, usize); m],
        p: [i32; 8]
    }

    let mut to = vec![HashSet::new(); 10];
    for (u, v) in uv {
        to[u].insert(v);
        to[v].insert(u);
    }
    let mut pp = [-1; 10];
    for i in 0..8 {
        pp[p[i] as usize] = i as i32 + 1;
    }
    let target = [-1, 1, 2, 3, 4, 5, 6, 7, 8, -1];
    let mut map: HashMap<[i32; 10], usize> = HashMap::new();
    let mut q: VecDeque<[i32; 10]> = VecDeque::new();
    map.insert(pp, 0);
    q.push_back(pp);
    while q.len() > 0 {
        let s = q.pop_front().unwrap(); // キューから取り出す
        let d = *map.get(&s).unwrap(); // そこに至るまでの試行回数
        for v in 1..=9 {
            if s[v] == -1 {
                // 交換可能な場所
                for u in &to[v] {
                    let mut new_s = s.clone();
                    new_s[*u] = s[v];
                    new_s[v] = s[*u];
                    if map.get(&new_s) == None {
                        map.insert(new_s, d + 1);
                        q.push_back(new_s);
                    }
                }
            }
        }
    }
    println!("{}", if let Some(a) = map.get(&target) { *a as i32 } else { -1 });
}

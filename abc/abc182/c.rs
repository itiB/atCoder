use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; n]; n]
    }

    // 1 -> x1 -> x2 ... -> 1 の順番
    let mut chosen = vec![false; n];
    let mut permutation = VecDeque::new();
    println!("{}", search(&mut permutation, &mut chosen, 0, &t, k));
}

fn search(permutation: &mut VecDeque<usize>, chosen: &mut Vec<bool>, mut ans: usize, t: &Vec<Vec<usize>>, k: usize) -> usize {
    // 0の分を除いておく
    if permutation.len() == chosen.len() - 1 {
        let mut p = permutation.clone();
        p.push_back(0);
        let mut past = 0;
        let mut s = 0;
        while let Some(next) = p.pop_front() {
            s += t[past][next];
            past = next;
        }
        if s == k { ans += 1 }
    } else {
        for i in 1..chosen.len() {
            if chosen[i] { continue }
            chosen[i] = true;
            permutation.push_back(i);
            ans = search(permutation, chosen, ans, t, k);
            chosen[i] = false;
            permutation.pop_back();
        }
    }
    return ans
}

use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
        k: usize
    }

    let mut permutation = VecDeque::new();
    let mut chosen = vec![false; s.len()];
    let mut list = BTreeSet::new();
    search(&mut permutation, &mut chosen, &s, &mut list);
    println!("{}", list.iter().nth(k - 1).unwrap());
}

fn search(permutation: &mut VecDeque<char>, chosen: &mut Vec<bool>, chars: &Vec<char>, list: &mut BTreeSet<String>) {
    if permutation.len() == chosen.len() {
        list.insert(permutation.clone().iter().collect::<String>());
    } else {
        for i in 0..chosen.len() {
            if chosen[i] { continue }
            chosen[i] = true;
            permutation.push_back(chars[i]);
            search(permutation, chosen, chars, list);
            chosen[i] = false;
            permutation.pop_back();
        }
    }
}

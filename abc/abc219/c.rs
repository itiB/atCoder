use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
fn main() {
    input! {
        x: Chars,
        n: usize,
        s: [Chars; n]
    }

    let di: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut map = HashMap::new();
    let mut c = 0;
    for y in x {
        map.insert(y, di[c]);
        c += 1;
    }
    let mut tmp = Vec::new();
    for ss in s {
        tmp.push((s2s(&ss, &map), ss));
    }
    tmp.sort();
    for (_, t) in tmp {
        let c: String = t.iter().collect();
        println!("{}", c);
    }
}

fn s2s(s: &Vec<char>, map: &HashMap<char, char>) -> String {
    let mut ans = Vec::new();
    for c in 0..s.len() {
        ans.push(*map.get(&s[c]).unwrap());
    }
    let a: String = ans.iter().collect();
    a
}
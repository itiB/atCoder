use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        s: Chars,
    }

    if !search(&s, a, c) || !search(&s, b, d) {
        println!("No");
        return;
    }

    if c > d && !tradable(&s, b, d) {
        println!("No");
        return;
    }

    println!("Yes");
}

fn search(map: &[char], start: usize, target: usize) -> bool {
    let mut count = 0;
    for i in start-1..=target {
        if i >= map.len() { break }
        if map[i] == '#' { 
            count += 1;
            if count == 2 { return false; }
        } else {
            count = 0;
        }
    }
    true
}

fn tradable(map: &[char], start: usize, target: usize) -> bool {
    for i in start..=target {
        if map[i - 2] == '.' && map[i - 1] == '.' && map[i] == '.' {
            return true;
        }
    }
    false
}
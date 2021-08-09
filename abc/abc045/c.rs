use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars
    }

    let mut subset = VecDeque::new();
    subset.push_back(0);
    println!("{}", search(1, 0, &mut subset, &s));
}

fn search(k: usize, mut sum: usize, subset: &mut VecDeque<usize>, s: &Vec<char>) -> usize {
    if k != s.len() {
        let tmp = sum;
        sum += search(k + 1, tmp, subset, s);
        subset.push_back(k);
        sum += search(k + 1, tmp, subset, s);
        subset.pop_back();
    } else {
        let mut last = s.len();
        let mut copy = subset.clone();
        while let Some(start) = copy.pop_back() {
            let mut c = 1;
            for i in (start..last).rev() {
                sum += (s[i] as usize - 48) * c;
                c *= 10;
            }
            last = start;
        }
    }
    return sum
}
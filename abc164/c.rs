use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i32,
        s: [String; n],
    }
    let set: HashSet<String> = s.into_iter().collect::<HashSet<String>>();
    println!("{}", set.len());
}
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        s: Chars
    }

    let mut set = HashSet::new();
    let mut large = false;
    let mut small = false;
    let a = 'a' as i32;
    let z = 'z' as i32;
    for c in s {
        if a <= c as i32 && c as i32 <= z {
            small = true;
        } else { large = true; }
        if set.contains(&c) {
            println!("No");
            return;
        }
        set.insert(c);
    }
    if large == true && small == true {
        println!("Yes");
    } else {
        println!("No");
    }
}
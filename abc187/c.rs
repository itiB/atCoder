use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut s_exp = HashSet::new();
    let mut s_non = HashSet::new();

    for mut st in s {
        if st.chars().nth(0) == Some('!') {
            s_exp.insert(st.split_off(1));
        } else {
            s_non.insert(st);
        }
    }
    for key in s_non {
        if s_exp.contains(&key) {
            println!("{}", key);
            return;
        }
    }
    println!("satisfiable");
}
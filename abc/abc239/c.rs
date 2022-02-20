use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64
    }

    let mut list = HashSet::new();
    list.insert((x1 + 2, y1 + 1));
    list.insert((x1 + 1, y1 + 2));
    list.insert((x1 - 1, y1 + 2));
    list.insert((x1 - 2, y1 + 1));
    list.insert((x1 - 2, y1 - 1));
    list.insert((x1 - 1, y1 - 2));
    list.insert((x1 + 1, y1 - 2));
    list.insert((x1 + 2, y1 - 1));

    let mut list2 = HashSet::new();
    list2.insert((x2 + 2, y2 + 1));
    list2.insert((x2 + 1, y2 + 2));
    list2.insert((x2 - 1, y2 + 2));
    list2.insert((x2 - 2, y2 + 1));
    list2.insert((x2 - 2, y2 - 1));
    list2.insert((x2 - 1, y2 - 2));
    list2.insert((x2 + 1, y2 - 2));
    list2.insert((x2 + 2, y2 - 1));

    for (x, y) in list {
        if list2.contains(&(x, y)) {
            println!("Yes");
            return
        }
    }
    println!("No");
}
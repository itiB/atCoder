use proconio::input;
use std::cmp::{min, max};

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut max_x1 = -1_000_000_001;
    let mut max_x2 = -1_000_000_001;
    let mut min_x1 = 1_000_000_001;
    let mut min_x2 = 1_000_000_001;
    let mut max_y1 = -1_000_000_001;
    let mut max_y2 = -1_000_000_001;
    let mut min_y1 = 1_000_000_001;
    let mut min_y2 = 1_000_000_001;

    let mut pass = false;

    for (x, y) in &xy {
        max_x1 = max(*x, max_x1);
        min_x1 = min(*x, min_x1);
        max_y1 = max(*y, max_y1);
        min_y1 = min(*y, min_y1);
    }
    let mut min_flagx = false;
    let mut max_flagx = false;
    let mut min_flagy = false;
    let mut max_flagy = false;
    for (x, y) in xy {
        if x <= max_x1 {
            if x < max_x1 || max_flagx == true {
                max_x2 = max(max_x2, x);
            }
            if x == max_x1 {
                max_flagx = true;
            }
        }
        if x >= min_x1 {
            if x > min_x1 || min_flagx == true {
                min_x2 = min(min_x2, x);
            }
            if x == min_x1 {
                min_flagx = true;
            }
        }

        if y <= max_y1 {
            if y < max_y1 || max_flagy == true {
                max_y2 = max(max_y2, y);
            }
            if y == max_y1 {
                max_flagy = true;
            }
        }
        if y >= min_y1 {
            if y > min_y1 || min_flagy == true {
                min_y2 = min(min_y2, y);
            }
            if y == min_y1 {
                min_flagy = true;
            }
        }
    }
    let mut v = Vec::new();
    v.push((max_x1 - min_x2).abs());
    v.push((max_x2 - min_x1).abs());
    v.push((max_x1 - min_x1).abs());
    v.push((max_y1 - min_y2).abs());
    v.push((max_y2 - min_y1).abs());
    if max_x1 != max_y1 {
        v.push((max_y1 - min_y1).abs());
    }
    v.sort();
    println!("{}", v[v.len() - 2]);
}

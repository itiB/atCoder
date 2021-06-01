use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        s: Chars,
        mut q: usize,
    }

    let mut queue = VecDeque::new();
    for c in s {
        queue.push_back(c);
    }

    let mut opposite = false;
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            opposite = !opposite;
        } else {
            input! {
                f: usize,
                c: char,
            }
            if (f == 1 && !opposite) || (f != 1 && opposite) {
                queue.push_front(c);
            } else {
                queue.push_back(c);
            }
        }
    }
    if !opposite {
        // while queue.len() > 0 {
        //     print!("{}", queue.pop_front().unwrap());
        // }
        println!("{}", queue.iter().collect::<String>());
    } else {
        // while queue.len() > 0 {
        //     print!("{}", queue.pop_back().unwrap());
        // }
        println!("{}", queue.iter().rev().collect::<String>());
    }
    // println!("");
}
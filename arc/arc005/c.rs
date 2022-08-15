use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h]
    }

    let mut sx: i32 = 0; // 始点
    let mut sy: i32 = 0;
    let mut gx: usize = 0;
    let mut gy: usize = 0;
    for y in 0..h {
        for x in 0..w {
            if c[y][x] == 's' {
                sx = x as i32;
                sy = y as i32;
            }
            if c[y][x] == 'g' {
                gx = x;
                gy = y;
            }
        }
    }

    // 上下左右の動きを表す
    let dx = vec![1, 0, -1, 0];
    let dy = vec![0, 1, 0, -1];

    let mut q: VecDeque<Node> = VecDeque::new();
    let mut dist = vec![vec![std::usize::MAX; w]; h];

    q.push_front(Node { x: sx, y: sy });
    dist[sy as usize][sx as usize] = 0;

    while let Some(node) = q.pop_front() {
        // 4つの動きの中から順番に
        for dir in 0..4 {
            let nx = node.x + dx[dir];
            let ny = node.y + dy[dir];

            if nx < 0 || nx >= w as i32 || ny < 0 || ny >= h as i32 {
                continue;
            }
            if c[ny as usize][nx as usize] != '#' {
                if dist[ny as usize][nx as usize] > dist[node.y as usize][node.x as usize] {
                    // 塀を壊す必要がなければ前にPush
                    q.push_front(Node { x: nx, y: ny });
                    dist[ny as usize][nx as usize] = dist[node.y as usize][node.x as usize];
                }
            } else {
                if dist[ny as usize][nx as usize] > dist[node.y as usize][node.x as usize] + 1 {
                    // 塀を壊す必要があれば後ろにPush
                    q.push_back(Node { x: nx, y: ny });
                    dist[ny as usize][nx as usize] = dist[node.y as usize][node.x as usize] + 1;
                }
            }
        }
    }

    println!("{}", if dist[gy][gx] > 2 { "NO" } else { "YES" });
}

#[derive(Debug)]
struct Node {
    x: i32,
    y: i32,
}

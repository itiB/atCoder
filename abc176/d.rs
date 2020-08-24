use proconio::input;
use std::collections::VecDeque;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: (usize, usize),
        d: (usize, usize),
        hw: [Chars; h],
    }

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut map = vec![vec![1000000 ; w]; h];
    q.push_front((c.0, c.1));
    map[c.0 - 1][c.1 - 1] = 0;

    let di = [-1, 0, 1, 0];
    let dj = [0, -1, 0, 1];

    while q.len() > 0 {
        let target = q.pop_front().expect("ERROR");
        let d = map[target.0 - 1][target.1 - 1];
        for v in 0..4 {
            let ni = target.0 as i32 - 1 + di[v];
            let nj = target.1 as i32 - 1 + dj[v];
            if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 { continue; }
            if hw[ni as usize][nj as usize] == '#' { continue; }
            if map[ni as usize][nj as usize] <= d { continue; }
            map[ni as usize][nj as usize] = d;
            q.push_front((ni as usize + 1, nj as usize + 1));
        }
        for ei in -2..=2 {
            for ej in -2..=2 {
                let ni = target.0 as i32 - 1 + ei;
                let nj = target.1 as i32 - 1 + ej;
                if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 { continue; }
                if hw[ni as usize][nj as usize] == '#' { continue; }
                if map[ni as usize][nj as usize] <= d + 1 { continue; }
                map[ni as usize][nj as usize] = d + 1;
                q.push_back((ni as usize + 1, nj as usize + 1));
            }
        }
    }
    if map[d.0 - 1][d.1 - 1] == 1000000 {
        println!("-1");
    } else {
        println!("{}", map[d.0 - 1][d.1 - 1]);
    }
}
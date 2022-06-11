use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(i64, i64); n]
    }
    let mut len = vec![1000001.0; n];
    for aa in a {
        // x: xy[a-1].0, y: xy[a-1].1
        for i in 0..n {
            let m = len[i] as f64;
            len[i] = m.min(length(xy[i].0, xy[i].1, xy[aa - 1].0, xy[aa - 1].1));
        }
    }
    let mut ans = 0.0;
    for f in len {
        if f > ans {
            ans = f;
        }
    }
    println!("{}", ans);
}

fn length(x1: i64, y1: i64, x2: i64, y2: i64) -> f64 {
    let x_len = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let y_len = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    return ((x_len * x_len + y_len * y_len) as f64).sqrt();
}

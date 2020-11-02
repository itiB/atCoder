use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    for i in 0..n {
        for j in i + 1..n {
            let x1 = xy[i].0 - xy[j].0;
            let y1 = xy[i].1 - xy[j].1;

            for v in j + 1..n {
                let x2 = xy[i].0 - xy[v].0;
                let y2 = xy[i].1 - xy[v].1;
                if x1 * y2 == y1 * x2 {
                    println!("Yes");
                    return
                }
            }
        }
    }
    println!("No");
}
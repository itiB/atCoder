use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut ans = (n * (n-1) * (n-2)) / 6;
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                if (xy[j].1 - xy[i].1) * (xy[k].0 - xy[i].0) == (xy[k].1 - xy[i].1) * (xy[j].0 - xy[i].0) {
                    ans -= 1;
                }
            }
        }
    }
    println!("{}", ans);
}
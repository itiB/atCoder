use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n]
    }

    let mut ans: f64 = 0.0;
    for i in 0..n {
        for j in i+1..n {
            let tmp = (xy[i].0 - xy[j].0).powf(2.0) + (xy[i].1 - xy[j].1).powf(2.0);
            ans = ans.max(tmp.sqrt());
        }
    }
    println!("{}", ans);
}
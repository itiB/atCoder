use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    let mut sum = 0;
    for i in 0..n {
        for j in i + 1..n {
            if (xy[j].1 - xy[i].1).abs() <= (xy[j].0 - xy[i].0).abs() {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}


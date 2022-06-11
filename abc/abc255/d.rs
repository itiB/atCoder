use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        x: [usize; q]
    }
    let max_a = a.iter().max().unwrap();
    let min_a = a.iter().min().unwrap();
    let mut max_sum = 0;
    let mut min_sum = 0;
    for aa in &a {
        max_sum += max_a - aa;
        min_sum += aa - min_a;
    }
    let s: usize = a.iter().sum();
    for xx in x {
        if xx >= *max_a {
            println!("{}", max_sum + (xx - max_a) * n);
        } else if xx <= *min_a {
            println!("{}", min_sum + (min_a - xx) * n);
        } else {
            //2分探索する
        }
    }
}

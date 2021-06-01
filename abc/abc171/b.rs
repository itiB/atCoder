use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [i32; n],
    }

    p.sort();
    let mut sum = 0;
    for a in 0..k {
        sum += p[a];
    }
    println!("{}", sum);
}
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut m = Vec::new();
    for i in 0..n {
        m.push((a[i], i));
    }
    m.sort();
    println!("{}", m[n-2].1 + 1);
}
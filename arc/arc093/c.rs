use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    a.push(0);
    let mut sum = 0;
    let mut last = 0;
    let mut memo = vec![0; n + 1];

    for i in 0..n + 1 {
        memo[i] = (a[i] - last).abs();
        sum += memo[i];
        last = a[i];
    }

    let mut aa = vec![0; 1];
    aa.append(&mut a);

    for i in 0..n {
        println!("{}", sum - memo[i] - memo[i+1] + (aa[i + 2] - aa[i]).abs());
    }
}
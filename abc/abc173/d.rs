use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    a.sort();
    a.reverse();
    let mut sum = a[0];
    let mut t = n - 2;
    for i in 1..n {
        let lim = 2;
        for _ in 0..lim {
            if t > 0 {
                sum += a[i];
                t -= 1;
            }
        }
    }
    println!("{}", sum);
}

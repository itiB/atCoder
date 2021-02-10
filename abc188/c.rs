use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 1 << n],
    }
    let half = 1 << n - 1;
    let mut l  = 0;
    let mut ll = 0;
    let mut r  = 0;
    let mut rr = 0;

    for i in 0..half {
        if l < a[i] {
            l = a[i];
            ll = i + 1;
        }
    }
    for j in half..a.len() {
        if r < a[j] {
            r = a[j];
            rr = j + 1;
        }
    }
    println!("{}", if l > r { rr } else { ll });
}
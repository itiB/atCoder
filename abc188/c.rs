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
    for i in half..1<<n - 1 {
        if r < a[i] {
            r = a[i];
            rr = i + 1;
        }
    }
    println!("{}", if l > r { ll } else { rr });
}
use proconio::input;

fn main() {
    input! {
        nn: i32,
        m: usize,
        t: i32,
    }

    let mut n = nn;

    let mut last = 0;
    for _ in 0..m {
        input! {
            a: i32,
            b: i32,
        }
        n -= a - last;
        if n <= 0 {
            println!("No");
            return;
        }
        n += b - a;
        if n > nn {
            n = nn;
        }
        last = b;
    }
    n -= t - last;
    if n <= 0 {
        println!("No");
        return;
    }
    println!("Yes");
}
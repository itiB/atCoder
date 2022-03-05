use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }

    for i in 1..n {
        if h[i - 1] >= h[i] {
            println!("{}", h[i-1]);
            return
        }
    }
    println!("{}", h[n-1]);
}
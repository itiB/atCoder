use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; 4*n-1]
    }

    a.sort();
    for i in 0..n-1 {
        if a[4*i] != a[4*i+1] || a[4*i] != a[4*i+2] || a[4*i] != a[4*i+3] {
            if a[4*i] == a[4*i+1] && a[4*i] == a[4*i+2] {
                println!("{}", a[4*i]);
            } else if a[4*i] == a[4*i+1] && a[4*i] == a[4*i+3] {
                println!("{}", a[4*i]);
            } else if a[4*i] == a[4*i+2] && a[4*i] == a[4*i+3]{
                println!("{}", a[4*i]);
            } else {
                println!("{}", a[4*i+1]);
            }
            return
        }
    }
    println!("{}", a[a.len()-1]);
}
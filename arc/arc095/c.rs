use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n]
    }

    let mut s = x.clone();
    s.sort();

    for i in x {
        println!("{}", if i < s[n / 2] {
            s[n / 2]
        } else {
            s[n / 2 - 1]
        });
    }
}
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n]
    }

    let mut ans = 0;
    for i in a {
        if i < p { ans += 1 }
    }
    println!("{}", ans);
}
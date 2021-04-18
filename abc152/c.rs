use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut min = p[0];
    let mut ans = 0;

    for i in p {
        if i <= min {
            min = i;
            ans += 1;
        }
    }
    println!("{}", ans);
}
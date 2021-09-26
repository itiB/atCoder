use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i128; n],
        x: i128
    }

    let all: i128 = a.iter().sum();
    let mut ans = (x / all) * (a.len() as i128);
    let mut sum = all * (x / all);
    for i in a {
        sum += i;
        ans += 1;
        if sum > x { break }
    }
    println!("{}", ans);
}
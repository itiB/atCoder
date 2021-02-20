use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = n;
    for _ in 0..k {
        ans = g(ans);
    }
    println!("{}", ans);
}

fn g(mut x: usize) -> usize {
    let mut v = Vec::new();
    let mut count = 1;
    while true {
        v.push(x % 10);
        x /= 10;
        if 0 <= x && x < 10 {
            v.push(x);
            count *= 10;
            break;
        }
        count *= 10;
    }
    let mut min = 0;
    let mut max = 0;
    v.sort();

    let mut count2 = 1;
    while let Some(num) = v.pop() {
        min += count2 * num;
        max += count * num;
        count2 *= 10;
        count /= 10;
    }
    max - min
}
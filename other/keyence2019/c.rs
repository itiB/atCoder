use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    let mut need = 0;
    let mut points = 0;
    let mut req = 0;
    let mut ans = 0;

    let mut f = Vec::new();

    for i in 0..n {
        need += b[i];
        points += a[i];

        if b[i] > a[i] {
            req += b[i] - a[i];
            ans += 1;
        } else {
            f.push(a[i] - b[i]);
        }
    }

    f.sort();
    f.reverse();

    if need > points {
        println!("-1");
    } else {
        for num in f {
            if req <= 0 { break; }
            req -= num;
            ans += 1;
        }
        println!("{}", ans);
    }
}
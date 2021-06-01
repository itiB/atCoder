use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        vp: [(usize, usize); n],
    }

    let mut a = 0;
    let mut ans = 0;
    for (wate, alco) in vp {
        ans += 1;
        a += wate * alco;
        if a > x * 100 {
            println!("{}", ans);
            return
        }
    }
    println!("-1");
}
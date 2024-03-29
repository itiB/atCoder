use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut ans = 0;
    let max_a: i64 = 200;
    let mut d = [0; 401]; // カウント用の配列
    for i in 0..n {
        // for j in i+1..n {
        //     ans += (a[i] - a[j]) * (a[i] - a[j]);
        // }
        // a[j]が同じになる個数を数えて高速化を図る
        for aj in -1 * max_a..=max_a {
            let c = d[(max_a + aj) as usize];
            let x = a[i] - aj;
            ans += x * x * c;
        }
        // いままで到達したところまでのがわかればループが回せる
        d[(max_a + a[i]) as usize]+= 1;
    }
    println!("{}", ans);
}

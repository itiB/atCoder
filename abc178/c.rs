use proconio::input;
     
fn main() {
    input! {
        n: i64,
    }
 
    let div = 1000000007;
    if n == 1 {
        println!("0");
    } else {
        println!("{}", expo(n, div));
    }
}
 
fn expo(time: i64, div: i64) -> i64 {
    let mut ans10 = 1;
    let mut ans9  = 1;
    let mut ans8  = 1;
    for _ in 0..time {
        ans10 *= 10;
        ans10 %= div;
        ans9 *= 9;
        ans9 %= div;
        ans8 *= 8;
        ans8 %= div;
    }
    let mut ans = (ans10 - ans9 - ans9 + ans8) % div;
    // 10 だけ% されたときに実行される
    if ans < 0 { ans += 1000000007; }
    ans
}
use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: i128,
    }
    let mut ans = n;

    for b in 0..60 {
        let powb = 2u64.pow(b as u32) as i128;
        let a = n / powb;
        let c = n - (n / powb) * powb;
        ans = min(ans, a + b + c);
    }
    println!("{}", ans);

    // let mut out  = 0;
    // // let mut safe = 1_000_000_000_000_000_000;
    // let mut safe = n;

    // while safe > out + 1 {
    //     let mid = out + (safe - out) / 2;
    //     let mut a = 0;
    //     let mut b = 0;
    //     let mut c = 0;
    //     let mut flag = true;
    //     for tmp_b in 0..60 {
    //             a = if tmp_b == 0 { 0 } else {n - mid + tmp_b / (2u64.pow(tmp_b as u32) as i128 - 1)};
    //             b = tmp_b;
    //             c = n - a * 2u64.pow(tmp_b as u32) as i128;
    //             // println!("{} {} {}", a, b, c);
    //             // println!("x: {}", a + b + c);
    //             if c >= 0 {
    //                 if mid > a + b + c {
    //                     println!("MIN!");
    //                     flag = false;
    //                 }
    //             }
    //         // }
    //     }
    //     // println!("{:?}", flag);
    //     if flag == true {
    //         safe = mid;
    //     } else {
    //         out = mid;
    //     }
    //     println!("out: {}, safe: {}", out, safe);
    // }
    // println!("{}", safe);
}

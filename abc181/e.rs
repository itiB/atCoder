use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [usize; n],
        w: [usize; m],
    }

    let mut ans: usize = 999_999_999;

    for me in w {
        let mut hs = h.clone();
        hs.push(me);
        hs.sort();
        let mut mid = 0;
        for i in 0..(n+1)/2 {
            mid += hs[i * 2 + 1] - hs[i * 2];
        }
        // let mut flag = 0;
        // let mut mid = 0;
        // for i in 0..(n+1) / 2 {
        //     if flag == 0 {
        //         if h[i + 1] > me {
        //             if h[i] > me {
        //                 mid += h[i] - me;
        //             } else {
        //                 mid += me - h[i];
        //             }
        //             flag = 1;
        //             // continue;
        //         }
        //     }
        //     mid += h[i + 1 + flag] - h[i + flag];
        // }
        // if flag == 0 {
        //     mid += if me > h[h.len() - 1] { me - h[h.len() - 1]} else {h[h.len() - 1] - me};
        // }
        ans = min(ans, mid);
    }
    println!("{}", ans);
}
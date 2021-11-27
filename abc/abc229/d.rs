use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        s: Chars,
        k: usize
    }

    let mut r = 0;
    let mut l = 0;
    let mut count: i32 = k as i32;
    for i in 0..s.len() {
        if s[i] == '.' {
            count -= 1;
        }
        if count < 0 {
            r = i - 1;
            break
        }
        if i == s.len() - 1 {
            println!("{}", s.len());
            return
        }
    }
    let mut ans = r - l + 1;
    let mut now = ans;
    // もしもすでに最後まで到達していたらAns
    if count > 0 {
        println!("{}", s.len());
        return
    }
    // println!("l: {}, r: {}, now: {}, ans: {}",l,r,now,ans);
    while r < s.len() - 1 {
        r += 1;
        now += 1;
        loop {
            // 連続してXなら続ける
            r += 1;
            now += 1;
            if r < s.len() {
                if s[r] == '.' {
                    r -= 1;
                    now -= 1;
                    break
                }
            } else {
                r -= 1;
                now -= 1;
                break
            }
        }
        // . になったらlを進める
        // l += 1;
        // now -= 1;
        loop {
            if s[l] == '.' {
                l += 1;
                now -= 1;
                break
            }
            l += 1;
            now -= 1;
        }
        ans = max(now, ans);
        // println!("l: {}, r: {}, now: {}, ans: {}",l,r,now,ans);
    }
    println!("{}", ans);
}
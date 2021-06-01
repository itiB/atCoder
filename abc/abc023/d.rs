use proconio::input;

fn main() {
    input! {
        n: usize,
        hs: [(usize, usize); n],
    }

    // 目標タイムをきめて二分探索する
    let mut left: usize = 0;
    let mut right: usize = 1_000_000_000;

    while right > left + 1 {
        // クリアできるタイムをRightにする
        let mid = left + (right - left) / 2;
        let mut check = true;

        let mut time = vec![0; n];

        for i in 0..n {
            if mid < hs[i].0 {
                // 高度が足りないもの
                check = false;
            } else {
                // 目標の高さまで何秒で到達するかを調べる
                time[i] = (mid - hs[i].0) / hs[i].1;
            }
        }
        // 残り時間が少ないものから撃っていく
        time.sort();
        for i in 0..n {
            if time[i] < i {
                check = false;
            }
         }
         if check {
             right = mid;
         } else {
             left = mid;
         }
    }
    println!("{}", right);
}

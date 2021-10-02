use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n]
    }

    let mut t = Vec::new();
    let mut ans = vec![0; n+1];
    for (a, b) in ab {
        t.push((a, true));
        t.push((a+b, false));
    }
    t.sort();
    let mut sum = 1;
    for i in 1..t.len() {
        if t[i].0 != t[i-1].0 {
            // もし時間が変わったならば...
            // 前の日付~いまの日付-1日目までsum人のプレイヤーがいた
            ans[sum] = ans[sum] + t[i].0 - t[i-1].0;
        }
        // この行で変化した人数を処理
        if t[i].1 {
            sum += 1;
        } else {
            sum -= 1;
        }
    }
    for i in 1..ans.len() - 1 {
        print!("{} ", ans[i])
    }
    println!("{}", ans[ans.len() - 1]);
}
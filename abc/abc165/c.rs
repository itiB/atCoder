use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i32,
        q: i32,
        abcd: [(usize, usize, i32, i32); q],
    }
    let mut ga = vec!{0; n};
    let ans = dfs(&abcd, &mut ga, 1, m, 0);
    println!("{}", ans);
}

fn dfs(abcd: &Vec<(usize, usize, i32, i32)>, ga: &mut Vec<i32>, lower: i32, m: i32, i: usize) -> i32 {
    if ga.len() == i {
        // 長さnまで到達したら終了，得点計算
        let mut score = 0;
        for &(a, b, c, d) in abcd {
            // 配列アクセスのため0スタートに合わせる
            if ga[b - 1] - ga[a - 1] == c {
                score += d;
            }
        }
        return score;
    }
    let mut ans = 0;
    // 最後の桁を"1つ前の値" ~ 最大値mまで変化させる
    for lastnum in lower..=m {
        ga[i] = lastnum;
        let t = dfs(abcd, ga, lastnum, m, i + 1);
        ans = if ans > t { ans } else { t };
    }
    ans
}
use proconio::input;

fn main() {
    input! {
        n: usize, // 1 <= N
        m: usize, // M <= 12
        x: i32, // 10 ^ 5
        c: [[i32; m + 1]; n],
    }
    let mut und = vec![0; m];

    for mid in c {
        for i in 0..m {
            und[i] += mid[i + 1];
        }
    }
    if und.iter().filter(|ga| ga <= &&x).count() > 0 {
        println!("-1");
        return
    }

    println!("{:?}", und);
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
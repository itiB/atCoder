use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    // 2本投げて起こりうる点数を列挙する
    a.push(0);
    let mut score12 = Vec::new();
    for i in 0..=n {
        for j in i..=n {
            score12.push(a[i] + a[j]);
        }
    }
    score12.sort();

    let mut ans = 0;
    for ab in &score12 {
        let mut left = 0;
        let mut right = &score12.len() - 1;

        // Mを超えない範囲で最大のものを2分探索
        if ab <= &m {
            while right > left + 1 {
                let mid = left + (right - left) / 2;
                if score12[mid] + ab < m {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            if score12[left] + ab > ans {
                // 既存のAnsよりも近ければ更新する
                ans = score12[left] + ab;
            }
        }
    }
    println!("{}", ans);
}

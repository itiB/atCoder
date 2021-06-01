use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    a.sort();
    b.push(0);
    b.push(1000_000_001);
    b.sort();
    let mut left = 0;
    let mut right = std::i64::MAX as usize;
    // let mut right = 100;

    while right > left + 1 {
        // Rは下にK個以上ある範囲の始まり
        // LはK個未満の数しか掛け算の結果がない者たち
        let mid = left + (right - left) / 2;

        if check(&a, &b, mid, k) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", right);
}

// 個数がK個以上になるならTrue
fn check(a: &[usize], b: &[usize], limit: usize, count: usize) -> bool {
    let mut sum = 0;
    for aa in a {
        // bについてaとかけてlimit未満の値を二分探索する
        let mut left = 0;
        let mut right = b.len() - 1;

        while right > left + 1 {
            let mid = left + (right - left) / 2;
            if aa * b[mid] > limit {
                right = mid;
            } else {
                left = mid;
            }
        }
        sum += left;
    }
    if sum >= count { false } else { true }
}